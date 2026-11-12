use clap::{Parser, Subcommand};


#[derive(Parser)]
#[clap(name = "todo", version = "0.1.0", author = "Darix SAMANI SIEW", about = "Rust-powered command-line todo manager with JSON persistence")]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add { task: String },
    List,
    Done { id: u32 },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Add { task } => {
            println!("Adding task: {}", task);
        }
        Command::List => {
            println!("Listing tasks...");
        }
        Command::Done { id } => {
            println!("Marking task as done: {}", id);
        }
    }
}
