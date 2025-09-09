use crate::{
    storage::{load_tasks, save_tasks},
    task::{Task, TaskError, TaskResult},
};
use chrono::NaiveDate;
use std::path::PathBuf;

/// Add Task
pub fn add_task(path: &PathBuf, description: String) -> TaskResult<()> {
    if description.trim().is_empty() {
        return Err(TaskError::InvalidInput(
            "Description cannot be empty".into(),
        ));
    }

    let mut tasks = load_tasks(path)?;
    let id = tasks.len() + 1;

    tasks.push(Task::new(id, description));
    save_tasks(path, &tasks)?;

    println!("Task {} added.", id);
    Ok(())
}

/// List Tasks
pub fn list_tasks(path: &PathBuf) -> TaskResult<()> {
    let tasks = load_tasks(path)?;

    if tasks.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }

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

/// Edit Task
pub fn edit_task(
    path: &PathBuf,
    id: usize,
    new_desc: Option<String>,
    new_due: Option<String>,
) -> TaskResult<()> {
    let mut tasks = load_tasks(path)?;

    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(TaskError::TaskNotFound(id))?;

    // Parse due date if provided
    let parsed_due = if let Some(due_str) = new_due {
        Some(
            NaiveDate::parse_from_str(&due_str, "%Y-%m-%d")
                .map_err(|e| TaskError::InvalidInput(format!("Invalid date: {}", e)))?,
        )
    } else {
        None
    };

    task.edit(new_desc, parsed_due);

    save_tasks(path, &tasks)?;
    println!("Task {} updated.", id);

    Ok(())
}

///Completed
pub fn complete_task(path: &PathBuf, id: usize) -> TaskResult<()> {
    let mut tasks = load_tasks(path)?;

    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(TaskError::TaskNotFound(id))?;

    //Mark the found task
    task.complete();
    //Savethe task
    save_tasks(path, &tasks)?;
    println!("Task {} completed", id);

    Ok(())
}

