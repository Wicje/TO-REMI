use crate::task::Task;
use std::{error::Error, fs, path::PathBuf};

pub fn load_tasks(path: &PathBuf) -> Result<Vec<Task>, Box<dyn Error>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(path)?;
    let tasks: Vec<Task> = serde_json::from_str(&data)?;
    Ok(tasks)
}

pub fn save_tasks(path: &PathBuf, tasks: &[Task]) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write(path, data)?;
    Ok(())
}
