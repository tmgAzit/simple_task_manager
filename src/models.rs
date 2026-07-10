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
