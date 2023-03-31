use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::{
    api::BoardTree,
    node::{directory::Directory, note::Note, project::Project, Node},
    utils::{get_mut_node, get_node},
};

#[derive(Serialize, Deserialize)]
pub struct Board {
    max_id: u64,
    nodes: Vec<Box<dyn Node>>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            max_id: 0,
            nodes: vec![Box::<Directory>::default()], // Root node has id 0
        }
    }
}

impl Board {
    pub fn load_or_create() -> Self {
        load().unwrap_or_default()
    }

    pub fn add_new_directory(&mut self, name: &str, parent_id: u64) -> Result<()> {
        let new_id = self.max_id + 1;
        let directory = Directory::with_name_and_id(name, new_id);
        get_mut_node(parent_id, &mut self.nodes)?.add_child(new_id)?;
        self.nodes.push(Box::new(directory));
        self.max_id = new_id;
        Ok(())
    }

    pub fn add_new_note(&mut self, name: &str, parent_id: u64) -> Result<()> {
        let new_id = self.max_id + 1;
        let note = Note::with_name_and_id(name, new_id);
        get_mut_node(parent_id, &mut self.nodes)?.add_child(new_id)?;
        self.nodes.push(Box::new(note));
        self.max_id = new_id;
        Ok(())
    }

    pub fn add_new_project(&mut self, name: &str, parent_id: u64) -> Result<()> {
        let new_id = self.max_id + 1;
        let project = Project::with_name_and_id(name, new_id);
        get_mut_node(parent_id, &mut self.nodes)?.add_child(new_id)?;
        self.nodes.push(Box::new(project));
        self.max_id = new_id;
        Ok(())
    }

    pub fn delete_node(&mut self, id: u64, parent_id: u64, recursive: bool) -> Result<()> {
        if id == 0 {
            return Err(anyhow!("Cannot delete root node"));
        }
        if !get_node(parent_id, &self.nodes)?
            .get_children()
            .ok_or(anyhow!("Cannot get child of node {parent_id}"))?
            .contains(&id)
        {
            return Err(anyhow!("Node {id} is not a child of node {parent_id}"));
        }
        if recursive {
            if let Some(children) = get_node(id, &self.nodes)?.get_children() {
                for child in children {
                    self.delete_node(child, id, true)?;
                }
            }
        } else if let Some(children) = get_node(id, &self.nodes)?.get_children() {
            for child in children {
                let parent = get_mut_node(parent_id, &mut self.nodes)?;
                parent.add_child(child)?;
            }
        }
        let parent = get_mut_node(parent_id, &mut self.nodes)?;
        parent.remove_child(id)?;
        self.nodes.retain(|n| n.get_id() != id);
        Ok(())
    }

    #[allow(clippy::borrowed_box)]
    pub fn get_node(&self, id: u64) -> Result<&Box<dyn Node>> {
        get_node(id, &self.nodes)
    }

    pub fn set_node_name(&mut self, id: u64, name: &str) -> Result<()> {
        get_mut_node(id, &mut self.nodes)?.set_name(name);
        Ok(())
    }

    pub fn set_note_content(&mut self, id: u64, content: &str) -> Result<()> {
        get_mut_node(id, &mut self.nodes)?
            .as_any_mut()
            .downcast_mut::<Note>()
            .ok_or(anyhow!("id {id} does not correspond to a Note node"))?
            .set_content(content);
        Ok(())
    }

    pub fn as_board_tree(&self) -> BoardTree {
        get_node(0, &self.nodes).unwrap().as_board_tree(&self.nodes)
    }

    pub fn save(&self) -> Result<()> {
        serde_json::to_writer_pretty(std::fs::File::create("board.json")?, &self)?;
        Ok(())
    }
}

fn load() -> Result<Board> {
    Ok(serde_json::from_reader(std::fs::File::open("board.json")?)?)
}

#[cfg(test)]
mod test {
    use super::Board;

    fn get_board() -> Board {
        Board::default()
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
    fn test_add_directory_valid() {
        let mut board = get_board();
        let result = board.add_new_directory("directory", 0);
        assert!(result.is_ok());
        assert!(board.nodes.len() == 2);
        assert!(&board.nodes[1].get_name() == "directory");
    }

    #[test]
    fn test_add_directory_invalid() {
        let mut board = get_board();
        let result = board.add_new_directory("directory", 1);
        assert!(result.is_err());
        assert!(board.nodes.len() == 1);
    }

    #[test]
    fn test_add_note_valid() {
        let mut board = get_board();
        let result = board.add_new_note("note", 0);
        assert!(result.is_ok());
        assert!(board.nodes.len() == 2);
        assert!(&board.nodes[1].get_name() == "note");
    }

    #[test]
    fn test_add_note_invalid() {
        let mut board = get_board();
        let result = board.add_new_note("note", 1);
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

    #[test]
    fn test_set_note_content_valid() {
        let mut board = get_board();
        board.add_new_note("note", 0).unwrap();
        board.set_note_content(1, "content").unwrap();
        assert!(
            board
                .get_node(1)
                .unwrap()
                .as_any()
                .downcast_ref::<super::Note>()
                .unwrap()
                .get_content()
                == "content"
        );
    }

    #[test]
    fn test_set_note_content_invalid() {
        let mut board = get_board();
        board.add_new_project("project", 0).unwrap();
        let result = board.set_note_content(1, "content");
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_node_valid() {
        let mut board = get_board();
        board.add_new_project("project", 0).unwrap();
        assert!(board.nodes.len() == 2);
        board.delete_node(1, 0, false).unwrap();
        assert!(board.nodes.len() == 1);
    }

    #[test]
    fn test_delete_node_invalid_id() {
        let mut board = get_board();
        board.add_new_project("project", 0).unwrap();
        assert!(board.nodes.len() == 2);
        let result = board.delete_node(2, 0, false);
        assert!(result.is_err());
        assert!(board.nodes.len() == 2);
    }

    #[test]
    fn test_delete_node_invalid_parent_id() {
        let mut board = get_board();
        board.add_new_project("project", 0).unwrap();
        assert!(board.nodes.len() == 2);
        let result = board.delete_node(1, 1, false);
        assert!(result.is_err());
        assert!(board.nodes.len() == 2);
    }

    #[test]
    fn test_delete_node_recursive() {
        let mut board = get_board();
        board.add_new_project("project", 0).unwrap();
        board.add_new_directory("directory", 0).unwrap();
        board.add_new_note("note", 2).unwrap();
        assert!(board.nodes.len() == 4);
        board.delete_node(2, 0, true).unwrap();
        assert!(board.nodes.len() == 2);
    }
}
