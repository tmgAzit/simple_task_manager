use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
   pub id: String,
   pub title: String,
   pub description: String,
   pub priority: Priority,
   pub due_date: Option<String>,
   pub status: Status,
   pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Pending,
    Completed,
}

impl Task {
    pub fn new(title: String, priority: Priority, due_date:Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description: String::new(),
            priority,
            due_date,
            status: Status::Pending,
            created_at: chrono::Local::now().to_rfc3339(), // DateTime stardardized string formate.
        }
    }
}
