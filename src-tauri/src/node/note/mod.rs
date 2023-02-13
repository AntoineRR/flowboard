use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::node::LeafError;

use super::Node;

#[derive(Serialize, Deserialize)]
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
