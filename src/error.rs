use std::fmt;

#[derive(Debug)]
pub enum TaskError {
    TaskNotFound(usize),
    InvalidDate(String),
    ReadError(std::io::Error),
    JsonError(serde_json::Error),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::TaskNotFound(id) => write!(f, "Task with id {} not found", id),
            TaskError::InvalidDate(s) => write!(f, "Invalid date format: {}", s),
            TaskError::IoError(e) => write!(f, "IO Error: {}", e),
            TaskError::JsonError(e) => write!(f, "JSON Error: {}", e),
        }
    }
}

impl std::error::Error for TaskError {}

impl From<std::io::Error> for TaskError {
    fn from(err: std::io::Error) -> Self {
        TaskError::IoError(err)
    }
}

impl From<serde_json::Error> for TaskError {
    fn from(err: serde_json::Error) -> Self {
        TaskError::JsonError(err)
    }
}

// Alias for Result
pub type TaskResult<T> = Result<T, TaskError>;
