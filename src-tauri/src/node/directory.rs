use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::Node;

#[derive(Serialize, Deserialize)]
pub struct Directory {
    id: u64,
    name: String,
    children: Vec<u64>,
}

impl Default for Directory {
    fn default() -> Self {
        Self {
            id: 0,
            name: "root".to_string(),
            children: vec![],
        }
    }
}

#[typetag::serde]
impl Node for Directory {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_children(&self) -> Option<Vec<u64>> {
        Some(self.children.clone())
    }

    fn add_child(&mut self, child_id: u64) -> Result<()> {
        self.children.push(child_id);
        Ok(())
    }

    fn remove_child(&mut self, child_id: u64) -> Result<()> {
        self.children.retain(|c| *c != child_id);
        Ok(())
    }
}
