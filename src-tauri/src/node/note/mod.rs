use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::{api::BoardTree, node::LeafError};

use super::{Node, NodeType};

#[derive(Clone, Serialize, Deserialize)]
pub struct Note {
    id: u64,
    name: String,
    content: String,
}

#[typetag::serde]
impl Node for Note {
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
        NodeType::Note
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
            node_type: NodeType::Note,
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

impl Note {
    pub fn with_name_and_id(name: &str, id: u64) -> Self {
        Self {
            id,
            name: name.into(),
            content: "".into(),
        }
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.into();
    }

    #[cfg(test)]
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}
