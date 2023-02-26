use std::{any::Any, error::Error, fmt::Display};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::BoardTree;

pub mod directory;
pub mod note;
pub mod project;

#[derive(Serialize, Deserialize)]
pub enum NodeType {
    Directory,
    Project,
    Note,
}

#[typetag::serde(tag = "type")]
pub trait Node: Send + Sync {
    fn get_id(&self) -> u64;
    fn get_name(&self) -> String;
    fn get_type(&self) -> NodeType;
    fn get_children(&self) -> Option<Vec<u64>>;
    fn add_child(&mut self, child_id: u64) -> Result<()>;
    fn remove_child(&mut self, child_id: u64) -> Result<()>;
    fn as_board_tree(&self, nodes: &[Box<dyn Node>]) -> BoardTree;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug)]
pub struct LeafError;

impl Display for LeafError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot add child to leaf node")
    }
}

impl Error for LeafError {}
