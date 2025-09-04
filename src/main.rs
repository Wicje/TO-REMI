use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

///simple cli tool
#[derive(Parser, Debug)]
#[command(name = " TODO CLI")]
#[command(about = " MAnage your todo from the terminal")]

struct Cli {
    #[command(Subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
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
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    //Path to Store todo
    let path = PathBuf::from("tasks.json");

    //Load existing task if available
    let mut tasks = load_task(&path)?;

    match cli.command {
        Commands::Add { task } => {
            tasks.push(Task {
                description: task,
                done: false,
            });
            save_tasks(&path, &tasks)?;
            println!("Task added!");
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No tasks yet!");
            } else {
                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.done { "[x]" } else { "[ ]" };
                    println!("{i} {status} {}", task.description);
                }
            }
        }
        Commands::Done { index } => {
            match tasks.get_mut(index) {
                Some(t) => {
                    t.done = true;
                    save_tasks(&path, &tasks)?;
                    println!("Task {index} marked as done!");
                }
                None => {
                    // return an error so we exit non-zero and show a friendly message
                    return Err(format!(
                        "Invalid index: {index} ({} tasks available)",
                        tasks.len()
                    )
                    .into());
                }
            }
        }
        Commands::Clear => {
            tasks.clear();
            save_tasks(&path, &tasks)?;
            println!("All tasks cleared!");
        }
    }

    Ok(())
}

fn load_tasks(path: &PathBuf) -> Result<Vec<Task>, Box<dyn Error>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string(path).map_err(|e| io_error("reading tasks file", e))?;
    let tasks: Vec<Task> = serde_json::from_str(&data)
        .map_err(|e| format!("parsing JSON in {}: {e}", path.display()))?;
    Ok(tasks)
}

fn save_tasks(path: &PathBuf, tasks: &[Task]) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string_pretty(tasks)
        .map_err(|e| format!("serializing tasks to JSON: {e}"))?;
    fs::write(path, data).map_err(|e| io_error("writing tasks file", e))?;
    Ok(())
}

fn io_error(ctx: &str, e: io::Error) -> Box<dyn Error> {
    format!("I/O error while {ctx}: {e}").into()
}
