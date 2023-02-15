use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum TaskStatus {
    Todo,
    Doing,
    Done,
    Archived,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    name: String,
    status: TaskStatus,
    description: String,
}
