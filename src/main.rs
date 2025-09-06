mod commands;
mod storage;
mod task;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "Todo CLI", about = "Manage your todos from the terminal")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add { description: String },
    List,
    // Edit, Clear, etc. will come later
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let path = PathBuf::from("tasks.json");

    match cli.command {
        Commands::Add { description } => commands::add_task(&path, description)?,
        Commands::List => commands::list_tasks(&path)?,
    }

    Ok(())
}

