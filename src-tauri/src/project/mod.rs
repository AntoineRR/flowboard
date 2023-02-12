mod task;

use serde::{Deserialize, Serialize};
use task::Task;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    tasks: Vec<Task>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: "New project".into(),
            tasks: Default::default(),
        }
    }
}

impl Project {
    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.into(),
            tasks: vec![],
        }
    }
}
