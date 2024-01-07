use chrono::NaiveDate;
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct LazyTodo {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
pub enum Bucket {
    TODAY,
    WEEK,
    MONTH,
    ALL,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Todo {
    pub id: i32,
    pub content: String,
    pub priority: bool,
    pub done: bool,
    pub created_at: NaiveDate,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoWrapper {
    pub todo: Vec<Todo>,
    pub counter: i32,
}
