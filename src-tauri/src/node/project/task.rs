use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum TaskStatus {
    TODO,
    DOING,
    DONE,
    ARCHIVED,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    name: String,
    status: TaskStatus,
    description: String,
}
