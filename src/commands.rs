use crate::{
    storage::{load_tasks, save_tasks},
    task::Task,
};
use chrono::NaiveDate;
use std::path::PathBuf;

//Add TAsk
pub fn add_task(path: &PathBuf, description: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = load_tasks(path)?;
    let id = tasks.len() + 1;
    tasks.push(Task::new(id, description));
    save_tasks(path, &tasks)?;
    println!("Task Added");
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

//Edit task
pub fn edit_task(
    path: &PathBuf,
    id: usize,
    new_desc: Option<String>,
    new_due: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = load_tasks(path)?;

    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        //Parse due date if provided
        let parsed_due = match new_due {
            Some(due_str) => Some(NaiveDate::parse_from_str(&due_str, "%Y-%m-%d")?),
            None => None,
        };

        task.edit(new_desc, parsed_due);
        save_tasks(path, &tasks)?;
        println!("Task {} updated", id);
    } else {
        println!("Task {} not found", id);
    }

    Ok(())
}

//
