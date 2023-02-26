use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
enum TaskStatus {
    Todo,
    Doing,
    Done,
    Archived,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    name: String,
    status: TaskStatus,
    description: String,
}
