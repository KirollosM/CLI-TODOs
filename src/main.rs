mod manager;
mod task;

use clap::{Parser, Subcommand};
use manager::TaskManager;

#[derive(Parser)]
#[command(name = "CLI-TODOs")]
#[command(about = "A simple CLI task manager written in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { title: String },
    List,
    Done { id: usize },
    Remove { id: usize },
}

fn main() {
    let cli = Cli::parse();

    let mut manager = TaskManager::new();

    match cli.command {
        Commands::Add { title } => {
            manager.add_task(title);
        }
        Commands::List => {
            manager.list_tasks();
        }
        Commands::Done { id } => {
            if let Err(e) = manager.complete_task(id) {
                eprintln!("Error: {}", e);
            }
        }
        Commands::Remove { id } => {
            if let Err(e) = manager.remove_task(id) {
                eprintln!("Error: {}", e);
            }
        }
    }
}
