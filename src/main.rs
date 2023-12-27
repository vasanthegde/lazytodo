

use std::{fs::{self, File}, path::Path, io::{Read, Write}, collections::HashMap};

use chrono::{Local, NaiveDate, Duration};
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};


fn main() {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let mut path = dirs::home_dir().unwrap();
    path.push(".lazytodo");
    fs::create_dir_all(&path).expect("failed to create config folder");
    
     let json_path = Path::new(&path).join("todo.json");
     let mut file = File::open(&json_path).expect("Unable to open file");
 
     // Read the file content into a string
     let mut contents = String::new();
     file.read_to_string(&mut contents).expect("Unable to read file");
     let mut todos: HashMap<String, Vec<Todo>> = serde_json::from_str(&contents).expect("Unable to parse JSON");

     println!("{:?}", todos);

    let todo = LazyTodo::parse();
    match todo.command.unwrap() {
      Commands::Add { todo, priority } =>{
        let entry = todos.entry(today).or_insert(Vec::new());
        entry.push(Todo { id: 1, content: todo, priority: priority.unwrap_or_default(), done: false })
        
      },

      Commands::List { range }=>{
        let ls : Vec<_> = todos.iter().filter(|x| isValidBucket(x.0.clone(), range.unwrap_or(Bucket::ALL))).flat_map(|x| x.1).collect();
        println!("{:?}", ls)
      },
      Commands::Done { id } =>{

      },
      Commands::Pset { id } => {

      },
      Commands::Clear { range } =>{

      }
    }

    let json_string = serde_json::to_string(&todos).expect("Failed to serialize HashMap to JSON");
    let mut file = File::create(json_path).expect("Failed to create file");
    file.write_all(json_string.as_bytes()).expect("Failed to write to file");


}

fn isValidBucket(key : String, bucket : Bucket) -> bool {
    let today = Local::now().naive_local().date();
    let key_date = NaiveDate::parse_from_str(&key, "%Y-%m-%d").expect("Failed to parse key date");

    match bucket {
        Bucket::TODAY => {
            return today == key_date
        },
        Bucket::WEEK =>{
            return key_date >= today.checked_sub_signed(Duration::days(7)).unwrap()
        },
        Bucket::MONTH =>{
            return key_date >= today.checked_sub_signed(Duration::days(30)).unwrap()
        }
        _ => {
            return false
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct LazyTodo {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        todo: String,
        #[arg(short, long)]
        priority : Option<bool>
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
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Bucket {
    TODAY,
    WEEK,
    MONTH,
    ALL
    
}

#[derive(Debug, Deserialize, Serialize)]
struct Todo{
    id : i32,
  content : String,
  priority : bool,
  done : bool
}
