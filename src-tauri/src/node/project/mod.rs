mod task;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use super::{LeafError, Node};
use task::Task;

#[derive(Serialize, Deserialize)]
pub struct Project {
    id: u64,
    pub name: String,
    tasks: Vec<Task>,
}

#[typetag::serde]
impl Node for Project {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_children(&self) -> Option<Vec<u64>> {
        None
    }

    fn add_child(&mut self, _child_id: u64) -> Result<()> {
        bail!(LeafError)
    }

    fn remove_child(&mut self, _child_id: u64) -> Result<()> {
        bail!(LeafError)
    }
}

impl Project {
    pub fn with_name_and_id(name: &str, id: u64) -> Self {
        Self {
            id,
            name: name.into(),
            tasks: vec![],
        }
    }
}
