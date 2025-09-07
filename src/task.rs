use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub completed: bool,
    pub due_date: Option<NaiveDate>,
}

impl Task {
    pub fn new(id: usize, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
            due_date: None,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn edit(&mut self, new_desc: Option<String>, new_due: Option<NaiveDate>) {
        if let Some(desc) = new_desc {
            self.description = desc;
        }

        if let Some(due) = new_due {
            self.due_date = Some(due);
        }
    }
}
