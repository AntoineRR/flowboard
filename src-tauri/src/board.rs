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
        self.nodes
            .iter_mut()
            .find(|n| n.get_id() == parent_id)
            .ok_or(anyhow!("No node found with id {parent_id}"))?
            .add_child(new_id)?;
        self.nodes.push(Box::new(project));
        self.max_id = new_id;
        Ok(())
    }

    pub fn get_children_ids(&self, parent_id: u64) -> Result<Vec<u64>> {
        self.nodes
            .iter()
            .find(|n| n.get_id() == parent_id)
            .ok_or(anyhow!("No node found with id {parent_id}"))?
            .get_children()
            .ok_or(anyhow!("Node with id {parent_id} cannot have children"))
    }

    pub fn get_names_for_ids(&self, ids: &[u64]) -> Vec<Option<String>> {
        ids.iter()
            .map(|id| match self.nodes.iter().find(|n| n.get_id() == *id) {
                Some(n) => Some(n.get_name()),
                None => None,
            })
            .collect()
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
    fn test_get_children_ids_valid() {
        let mut board = get_board();
        board.add_new_project("project1", 0).unwrap();
        board.add_new_project("project2", 0).unwrap();
        board.add_new_project("project3", 0).unwrap();
        let ids = board.get_children_ids(0).unwrap();
        assert!(ids == vec![1, 2, 3]);
    }

    #[test]
    fn test_get_children_ids_invalid() {
        let mut board = get_board();
        board.add_new_project("project", 0).unwrap();
        let result = board.get_children_ids(1);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_names_for_ids_valid() {
        let mut board = get_board();
        board.add_new_project("project1", 0).unwrap();
        board.add_new_project("project2", 0).unwrap();
        board.add_new_project("project3", 0).unwrap();
        let names = board.get_names_for_ids(&[1, 3]);
        assert!(names == vec![Some("project1".to_string()), Some("project3".to_string())]);
    }

    #[test]
    fn test_get_names_for_ids_invalid() {
        let mut board = get_board();
        board.add_new_project("project1", 0).unwrap();
        board.add_new_project("project2", 0).unwrap();
        board.add_new_project("project3", 0).unwrap();
        let names = board.get_names_for_ids(&[1, 3, 5]);
        assert!(
            names
                == vec![
                    Some("project1".to_string()),
                    Some("project3".to_string()),
                    None
                ]
        );
    }
}
