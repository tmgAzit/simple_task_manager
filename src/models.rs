use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Local};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: String,
    title: String,
    description: String,
    priority: Priority,
    due_date: Option<String>,
    status: Status,
    created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Status {
    Pending,
    Completed,
}

impl Task {
    pub fn new(title: String, priority: Priority, due_date: Option<String> ) -> Self {
    Self {
        id: uuid::Uuid::new_v4().to_string(),
        title,
        description: String::new(),
        priority,
        due_date,
        status: Status::Pending,
        created_at: chrono::Local::now()::to_rfc(3330) 
    }
}

