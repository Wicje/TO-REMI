use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

///simple cli tool
#[derive(Parser)]
#[command(name = " TODO CLI")]
#[command(about = " MAnage your todo from the terminal")]

struct Cli {
    #[command(Subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    //Add a task
    Add { task: String },

    //list all task
    List,

    //Mark a task as done
    Done { index: usize },

    //clearall task
    Clear,
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    done: bool,
}

fn main() {
    let cli = Cli::parse();

    //path to storee todo
    let path = PathBuf::from("tasks.json");

    //load existing task  or empmty list if missing
    let mut tasks: Vec<Task> = load_tasks(&path);

    match cli.command {
        Commands::Add { task } => {
            task.push(Task {
                description: task,
                done: :false,
            });
            save_tasks(&path, &task);
            println!("Task Added");
        }

        Commands::List => {
            if tasks.is_empty() {
                println!("No Task");
            } else {
                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.done { "[x]" } else { "[ ]" };
                    println!("{} {} {}", i, status, task.description);
                }
            }
        }

        Commands::Done { index } => {
            if index < tasks.len() {
                tasks[index].done = true;
                save_tasks(&path, &tasks);
                println!("Task {} marked as done", index);
            } else {
                eprintln!("Invalid index: {}", index);
            }
        }

        Commands::Clear => {
            tasks.clear();
            save_task(&path, &tasks);
            println!("All tasked Cleared");
        }
    }
}

fn load_tasks(path: &PathBuf) -> Vec<Task> {
    if !path.exists() {
        return Vec::new();
    }
    let data = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

fn save_tasks(path: &PathBuf, tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    fs::write(path, data).expect("Failed to save tasks");
}
