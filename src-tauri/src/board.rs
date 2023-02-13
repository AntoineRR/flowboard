use std::collections::HashSet;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::node::{directory::Directory, project::Project, Node};

#[derive(Serialize, Deserialize)]
pub struct Board {
    max_id: u64,
    nodes: Vec<Box<dyn Node>>,
}

impl Default for Board {
    fn default() -> Self {
        Self::load_or_create()
    }
}

impl Board {
    pub fn load_or_create() -> Self {
        Self {
            max_id: 0,
            nodes: vec![Box::new(Directory::default())], // Root node has id 0
        }
    }

    pub fn add_new_project(&mut self, name: &str, parent_id: u64) -> Result<()> {
        let new_id = self.max_id + 1;
        let project = Project::with_name_and_id(name, new_id);
        self.nodes.push(Box::new(project));
        self.nodes
            .iter_mut()
            .find(|n| n.get_id() == parent_id)
            .ok_or(anyhow!("No node found with id {parent_id}"))?
            .add_child(new_id)
            .unwrap();
        self.max_id = new_id;
        Ok(())
    }

    pub fn get_children_ids(&self, parent_id: u64) -> Result<Vec<u64>> {
        self.nodes
            .iter()
            .find(|n| n.get_id() == parent_id)
            .ok_or(anyhow!("No node found with id {parent_id}"))?
            .get_children()
            .ok_or(anyhow!("No children found for node with id {parent_id}"))
    }

    pub fn get_names_for_ids(&self, ids: &[u64]) -> Vec<String> {
        let ids_hashset = HashSet::<_>::from_iter(ids.iter());
        self.nodes
            .iter()
            .filter(|n| ids_hashset.contains(&n.get_id()))
            .map(|n| n.get_name())
            .collect()
    }
}
