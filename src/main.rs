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
    Add {
        description: String,
    },
    List,
    Edit {
        id: usize,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        due: Option<String>,
    },
    Complete {
        id: usize,
    },
    // Edit, Clear, etc. will come later
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let path = PathBuf::from("tasks.json");

    match cli.command {
        Commands::Add { description } => commands::add_task(&path, description)?,
        Commands::List => commands::list_tasks(&path)?,
        Commands::Edit {
            id,
            description,
            due,
        } => commands::edit_task(&path, id, description, due)?,
        Commands::Complete { id } => commands::complete_task(&path, id)?,
    }

    Ok(())
}
