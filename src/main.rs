#[macro_use]
extern crate prettytable;
use colored::Colorize;
use lazytodo::{Bucket, Commands, LazyTodo, Todo, TodoWrapper};
use prettytable::Table;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use chrono::{Duration, Local, NaiveDate};
use clap::Parser;
use itertools::Itertools;

fn main() {
    let today = Local::now().date_naive();
    let mut todo_config_path = dirs::home_dir().unwrap();
    todo_config_path.push(".lazytodo");
    let todo_file_path = Path::new(&todo_config_path).join("todo.json");

    if !todo_file_path.exists() {
        fs::create_dir_all(&todo_config_path).expect("Failed to create config folder");
        File::create(&todo_file_path).expect("Unable to create todo.json file");
        write_todo_wrapper(
            TodoWrapper {
                todo: vec![],
                counter: 0,
            },
            &todo_file_path,
        )
    }

    let mut file = File::open(&todo_file_path).expect("Unable to open todo json file");

    // Read the file content into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read todo json file");
    let mut wrapper: TodoWrapper =
        serde_json::from_str(&contents).expect("Unable to parse todo file content");

    let total_todos = wrapper.todo.len();
    // clear old todos which are greater than 30 days
    wrapper
        .todo
        .retain(|x| x.created_at >= today.checked_sub_signed(Duration::days(30)).unwrap());

    if wrapper.todo.len() < total_todos {
        println!("Deleted {} old todos", total_todos - wrapper.todo.len())
    }
    let cli = LazyTodo::parse();
    match cli.command {
        Commands::Add { todo, priority } => {
            add_todo(&mut wrapper, todo, priority, today);
        }

        Commands::List { range } => {
            list_todos(&wrapper, range);
        }

        Commands::Done { id } => {
            if let Some(todo) = wrapper.todo.iter_mut().find(|x| x.id == id) {
                todo.done = true
            }
        }
        Commands::Pset { id } => {
            if let Some(todo) = wrapper.todo.iter_mut().find(|x| x.id == id) {
                todo.priority = true
            }
        }
        Commands::Clear { range } => wrapper
            .todo
            .retain(|x| !is_valid_bucket(x.created_at, range)),
        Commands::Delete { id } => wrapper.todo.retain(|x| x.id != id),
    }

    write_todo_wrapper(wrapper, &todo_file_path)
}

fn list_todos(wrapper: &TodoWrapper, range: Option<Bucket>) {
    let mut group: HashMap<&NaiveDate, Vec<&Todo>> = HashMap::new();
    let ls: Vec<_> = wrapper
        .todo
        .iter()
        .filter(|x| is_valid_bucket(x.created_at.clone(), range.unwrap_or(Bucket::ALL)))
        .collect();

    for todo in ls {
        let entry = group.entry(&todo.created_at).or_insert(Vec::new());
        entry.push(todo);
    }

    let table = create_table(group);
    table.printstd();
}

fn create_table(group: HashMap<&NaiveDate, Vec<&Todo>>) -> Table {
    let mut table = Table::new();
    table.add_row(row![
        "ID".bold().cyan(),
        "TODO_NAME".bold().cyan(),
        "CREATED_AT".bold().cyan()
    ]);
    for todos_grouped in group.iter().sorted() {
        for todo_item in todos_grouped.1 {
            let content = if todo_item.done && todo_item.priority {
                todo_item
                    .content
                    .to_uppercase()
                    .strikethrough()
                    .italic()
                    .dimmed()
                    .red()
            } else if todo_item.done {
                todo_item
                    .content
                    .to_uppercase()
                    .strikethrough()
                    .italic()
                    .dimmed()
            } else if todo_item.priority {
                todo_item.content.to_uppercase().red()
            } else {
                todo_item.content.to_uppercase().normal()
            };
            table.add_row(row![todo_item.id, content, todo_item.created_at]);
        }
    }
    table
}

fn add_todo(wrapper: &mut TodoWrapper, todo: String, priority: Option<bool>, today: NaiveDate) {
    wrapper.counter += 1;
    wrapper.todo.push(Todo {
        id: wrapper.counter,
        content: todo,
        priority: priority.unwrap_or_default(),
        done: false,
        created_at: today,
    })
}

fn write_todo_wrapper(todo_wrapper: TodoWrapper, todo_file_path: &PathBuf) {
    let json_string = serde_json::to_string(&todo_wrapper).expect("Failed to deserialize to JSON");
    let mut file = File::create(todo_file_path).expect("Failed to create file");
    file.write_all(json_string.as_bytes())
        .expect("Failed to write to file");
}

fn is_valid_bucket(key: NaiveDate, bucket: Bucket) -> bool {
    let today = Local::now().naive_local().date();
    match bucket {
        Bucket::TODAY => return today == key,
        Bucket::WEEK => return key >= today.checked_sub_signed(Duration::days(7)).unwrap(),
        Bucket::MONTH => return key >= today.checked_sub_signed(Duration::days(30)).unwrap(),
        Bucket::ALL => return true,
    }
}
