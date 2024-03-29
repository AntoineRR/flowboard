use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{api::BoardTree, utils::get_node};

use super::{Node, NodeType};

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

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn get_type(&self) -> NodeType {
        NodeType::Directory
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

    fn as_board_tree(&self, nodes: &[Box<dyn Node>]) -> BoardTree {
        BoardTree {
            id: self.id,
            node_type: NodeType::Directory,
            name: self.name.clone(),
            children: self
                .children
                .iter()
                .map(|id| get_node(*id, nodes).unwrap().as_board_tree(nodes))
                .collect(),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Directory {
    pub fn with_name_and_id(name: &str, id: u64) -> Self {
        Self {
            id,
            name: name.into(),
            children: vec![],
        }
    }
}
