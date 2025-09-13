
use dirs::data_dir;
use std::path::{PathBuf};

pub fn default_data_file() -> PathBuf {
    let mut dir = data_dir().unwrap_or_else(|| {
        // fallback to current directory
        std::env::current_dir().expect("failed to get current dir")
    });
    dir.push("toremi");                // app folder
    std::fs::create_dir_all(&dir).ok(); // ignore if exists
    dir.push("tasks.json");
    dir
}

