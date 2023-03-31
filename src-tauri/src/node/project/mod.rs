mod task;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::api::BoardTree;

use super::{LeafError, Node, NodeType};
use task::Task;

#[derive(Clone, Serialize, Deserialize)]
pub struct Project {
    id: u64,
    name: String,
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

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn get_type(&self) -> NodeType {
        NodeType::Project
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

    fn as_board_tree(&self, _nodes: &[Box<dyn Node>]) -> BoardTree {
        BoardTree {
            id: self.id,
            node_type: NodeType::Project,
            name: self.name.clone(),
            children: vec![],
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
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
