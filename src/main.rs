

use clap::{Parser, Subcommand, ValueEnum};

fn main() {

    let todo = LazyTodo::parse();
    match todo.command.unwrap() {
      Commands::Add { todo, priority } =>{
        
      },
      Commands::List { range }=>{

      },
      Commands::Done { id } =>{

      },
      Commands::Pset { id } => {

      },
      Commands::Clear { range } =>{

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
    MONTH
    
}
