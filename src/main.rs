#[macro_use]
extern crate prettytable;
use colored::Colorize;
use prettytable::Table;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use chrono::{Duration, Local, NaiveDate};
use clap::{Parser, Subcommand, ValueEnum};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

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
        println!("Cleared {} old todos", total_todos - wrapper.todo.len())
    }
    let cli = LazyTodo::parse();
    match cli.command {
        Commands::Add { todo, priority } => {
            wrapper.counter += 1;
            wrapper.todo.push(Todo {
                id: wrapper.counter,
                content: todo,
                priority: priority.unwrap_or_default(),
                done: false,
                created_at: today,
            })
        }

        Commands::List { range } => {
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

            table.printstd();
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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct LazyTodo {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        todo: String,
        #[arg(short, long)]
        priority: Option<bool>,
    },
    List {
        range: Option<Bucket>,
    },
    Clear {
        range: Bucket,
    },
    Done {
        id: i32,
    },
    Pset {
        id: i32,
    },
    Delete {
        id: i32,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Bucket {
    TODAY,
    WEEK,
    MONTH,
    ALL,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct Todo {
    id: i32,
    content: String,
    priority: bool,
    done: bool,
    created_at: NaiveDate,
}

#[derive(Debug, Deserialize, Serialize)]
struct TodoWrapper {
    todo: Vec<Todo>,
    counter: i32,
}
