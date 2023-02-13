use std::{error::Error, fmt::Display};

use anyhow::Result;

pub mod directory;
pub mod note;
pub mod project;

#[typetag::serde(tag = "type")]
pub trait Node: Send + Sync {
    fn get_id(&self) -> u64;
    fn get_name(&self) -> String;
    fn get_children(&self) -> Option<Vec<u64>>;
    fn add_child(&mut self, child_id: u64) -> Result<()>;
    fn remove_child(&mut self, child_id: u64) -> Result<()>;
}

#[derive(Debug)]
pub struct LeafError;

impl Display for LeafError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot add child to leaf node")
    }
}

impl Error for LeafError {}
