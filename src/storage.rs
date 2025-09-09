use crate::error::TaskError;
use crate::task::Task;
use std::fs;
use std::path::PathBuf;

pub fn load_tasks(path: &PathBuf) -> Result<Vec<Task>, TaskError> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string(path)?; // ? automatically converts to TaskError via From

    let tasks: Vec<Task> = serde_json::from_str(&data)?; // ? automatically converts to TaskError

    Ok(tasks)
}

pub fn save_tasks(path: &PathBuf, tasks: &Vec<Task>) -> Result<(), TaskError> {
    let data = serde_json::to_string_pretty(tasks)?; // ? handles JsonError
    fs::write(path, data)?; // ? handles IoError
    Ok(())
}
