use anyhow::{anyhow, Result};

use crate::node::Node;

#[allow(clippy::borrowed_box)]
pub fn get_node(id: u64, nodes: &[Box<dyn Node>]) -> Result<&Box<dyn Node>> {
    nodes
        .iter()
        .find(|n| n.get_id() == id)
        .ok_or(anyhow!("No node found with id {id}"))
}

pub fn get_mut_node(id: u64, nodes: &mut [Box<dyn Node>]) -> Result<&mut Box<dyn Node>> {
    nodes
        .iter_mut()
        .find(|n| n.get_id() == id)
        .ok_or(anyhow!("No node found with id {id}"))
}
