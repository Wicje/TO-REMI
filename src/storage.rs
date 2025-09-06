use crate::task::Task;
use std::{error::Error, fs, path::PathBuf};

pub fn load_tasks(path: &PathBuf) -> Result<Vec<Task>, Box<dyn Error>> {
    if !path.exists() {}
}
