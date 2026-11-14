use std::fs;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(
    name = "todo",
    version = "0.1.0",
    author = "Darix SAMANI SIEWE",
    about = "Rust-powered command-line todo manager with JSON persistence"
)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: u32,
    name: String,
    done: bool,
}

#[derive(Subcommand)]
enum Command {
    Add {
        task: String,
    },
    List {
        #[clap(long, help = "Show only pending tasks")]
        pending: bool,
        #[clap(long, help = "Show only done tasks")]
        done: bool,
    },
    Done {
        id: u32,
    },
    Edit {
        id: u32,
        task: String,
    },
}
fn main() {
    let cli = Cli::parse();
    let store = std::env::var("TODO_STORE").unwrap_or_else(|_| "todos.json".to_string());

    match cli.command {
        Command::Add { task } => {
            let content = fs::read_to_string(&store).unwrap_or_else(|_| "[]".to_string());
            let mut todos: Vec<Todo> = serde_json::from_str(&content).unwrap_or_default();

            let todo = Todo {
                id: todos.len() as u32 + 1,
                name: task,
                done: false,
            };
            todos.push(todo.clone());
            fs::write(&store, serde_json::to_string_pretty(&todos).unwrap()).unwrap();
            println!("✅ Added: {} (#{})", todo.name, todo.id);
        }
        Command::List { pending, done } => {
            let content = fs::read_to_string(&store).unwrap_or_else(|_| "[]".to_string());
            let todos: Vec<Todo> = serde_json::from_str(&content).unwrap_or_default();

            if todos.is_empty() {
                println!("No tasks found.");
            } else {
                println!("Tasks:");
                for todo in &todos {
                    let should_display = if pending && done {
                        true
                    } else if pending {
                        !todo.done
                    } else if done {
                        todo.done
                    } else {
                        true
                    };

                    if should_display {
                        let status = if todo.done { "✅" } else { "❌" };
                        println!("{} {}: {}", status, todo.id, todo.name);
                    }
                }
            }
        }
        Command::Done { id } => {
            let content = fs::read_to_string(&store).unwrap_or_else(|_| "[]".to_string());
            let mut todos: Vec<Todo> = serde_json::from_str(&content).unwrap_or_default();

            let mut found = false;
            for todo in &mut todos {
                if todo.id == id {
                    todo.done = true;
                    found = true;
                    break;
                }
            }

            if found {
                fs::write(&store, serde_json::to_string_pretty(&todos).unwrap()).unwrap();
                println!("✅ Task marked as done: {}", id);
            } else {
                eprintln!("Task id {} doesn't exist", id);
            }
        }
        Command::Edit { id, task } => {
            let content = fs::read_to_string(&store).unwrap_or_else(|_| "[]".to_string());
            let mut todos: Vec<Todo> = serde_json::from_str(&content).unwrap_or_default();

            let mut found = false;
            for todo in &mut todos {
                if todo.id == id {
                    todo.name = task;
                    found = true;
                    break;
                }
            }

            if found {
                fs::write(&store, serde_json::to_string_pretty(&todos).unwrap()).unwrap();
                println!("✅ Task edited: {}", id);
            } else {
                eprintln!("Task id {} doesn't exist", id);
            }
        }
    }
}
