use std::fs;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(
    name = "todo",
    version = "0.1.0",
    author = "Darix SAMANI SIEW",
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
    Add { task: String },
    List,
    Done { id: u32 },
    Edit { id: u32, task: String },
}
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Add { task } => {
            let content = fs::read_to_string("todos.json").unwrap_or_else(|_| "[]".to_string());

            let mut todos: Vec<Todo> = serde_json::from_str(&content).unwrap_or_default();

            let todo = Todo {
                id: todos.len() as u32 + 1,
                name: task,
                done: false,
            };
            todos.push(todo.clone());
            fs::write("todos.json", serde_json::to_string_pretty(&todos).unwrap()).unwrap();
            println!("✅ Added: {} (#{})", todo.name, todos.len());
        }
        Command::List => {
            let content = fs::read_to_string("todos.json").unwrap_or_else(|_| "[]".to_string());

            let todos: Vec<Todo> = serde_json::from_str(&content).unwrap_or_default();

            if todos.is_empty() {
                println!("No tasks found.");
            } else {
                println!("Tasks:");
                for todo in todos {
                    let status = if todo.done { "✅" } else { "❌" };
                    println!("{} {}: {}", status, todo.id, todo.name);
                }
            }
        }
        Command::Done { id } => {
            let content = fs::read_to_string("todos.json").unwrap_or_else(|_| "[]".to_string());

            let mut todos: Vec<Todo> = serde_json::from_str(&content).unwrap_or_default();

            if !todos.is_empty() && id < todos.len() as u32 {
                for todo in &mut todos {
                    if todo.id == id {
                        todo.done = true;
                        break;
                    }
                }

                fs::write("todos.json", serde_json::to_string_pretty(&todos).unwrap()).unwrap();
                println!("✅ Task marked as done: {}", id);
            } else {
                eprintln!("Task is empty or id {} doesn't exist", id);
            }
        }
        Command::Edit { id, task } => {
            let content = fs::read_to_string("todos.json").unwrap_or_else(|_| "[]".to_string());

            let mut todos: Vec<Todo> = serde_json::from_str(&content).unwrap_or_default();

            if !todos.is_empty() && id < todos.len() as u32 {
                for todo in &mut todos {
                    if todo.id == id {
                        todo.name = task;
                        break;
                    }
                }

                fs::write("todos.json", serde_json::to_string_pretty(&todos).unwrap()).unwrap();
                println!("✅ Task edited: {}", id);
            } else {
                eprintln!("Task is empty or id {} doesn't exists", id);
            }
        }
    }
}
