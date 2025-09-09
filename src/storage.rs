use crate::error::TaskError;
use crate::task::Task;
use std::fs;
use std::path::PathBuf;

pub fn load_tasks(path: &PathBuf) -> Result<Vec<Task>, TaskError> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string(path).map_err(|e| TaskError::ReadError(e.to_string()))?;

    let tasks: Vec<Task> =
        serde_json::from_str(&data).map_err(|e| TaskError::ParseError(e.to_string()))?;

    Ok(tasks)
}

pub fn save_tasks(path: &PathBuf, tasks: &Vec<Task>) -> Result<(), TaskError> {
    let data =
        serde_json::to_string_pretty(tasks).map_err(|e| TaskError::ParseError(e.to_string()))?;

    fs::write(path, data).map_err(|e| TaskError::WriteError(e.to_string()))?;

    Ok(())
}
