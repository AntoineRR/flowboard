use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    api::BoardTree,
    node::{directory::Directory, project::Project, Node},
    utils::{get_mut_node, get_node},
};

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
            nodes: vec![Box::<Directory>::default()], // Root node has id 0
        }
    }

    pub fn add_new_project(&mut self, name: &str, parent_id: u64) -> Result<()> {
        let new_id = self.max_id + 1;
        let project = Project::with_name_and_id(name, new_id);
        get_mut_node(parent_id, &mut self.nodes)?.add_child(new_id)?;
        self.nodes.push(Box::new(project));
        self.max_id = new_id;
        Ok(())
    }

    pub fn as_board_tree(&self) -> BoardTree {
        get_node(0, &self.nodes).unwrap().as_board_tree(&self.nodes)
    }
}

#[cfg(test)]
mod test {
    use super::Board;

    fn get_board() -> Board {
        Board::load_or_create()
    }

    #[test]
    fn test_new_board() {
        let board = get_board();
        assert!(board.nodes.len() == 1);
        assert!(&board.nodes[0].get_name() == "root");
    }

    #[test]
    fn test_add_project_valid() {
        let mut board = get_board();
        let result = board.add_new_project("project", 0);
        assert!(result.is_ok());
        assert!(board.nodes.len() == 2);
        assert!(&board.nodes[1].get_name() == "project");
    }

    #[test]
    fn test_add_project_invalid() {
        let mut board = get_board();
        let result = board.add_new_project("project", 1);
        assert!(result.is_err());
        assert!(board.nodes.len() == 1);
    }

    #[test]
    fn test_as_board_tree() {
        let mut board = get_board();
        board.add_new_project("project1", 0).unwrap();
        board.add_new_project("project2", 0).unwrap();
        board.add_new_project("project3", 0).unwrap();
        let board_tree = board.as_board_tree();
        assert!(board_tree.children.len() == 3);
        for (id, child) in board_tree.children.iter().enumerate() {
            assert!(child.name == format!("project{}", id + 1));
        }
    }
}
