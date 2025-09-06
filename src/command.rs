use crate::{
    storage::{load_tasks, save_tasks},
    task::Task,
};
use chrono::NaiveDate;
use std::path::PathBuf;

//Add TAsk
pub fn add_task(path: &Pathbuf, description; String) -> Result<(), Box<dyn Error> {
    let mut tasks = load_tasks(path)?;
    let id = tasks.len() +1
    task.push(Task::new(id, description));
    save_tasks(path, &tasks)?;
    println("Task Added");
    Ok(())
}

//List TAsk
pub fn list_tasks(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let tasks = load_tasks(path)?;
    for task in tasks {
        println!(
            "[{}] {} - {}",
            if task.completed { "x" } else { " " },
            task.id,
            task.description
        );
    }
    Ok(())
}
