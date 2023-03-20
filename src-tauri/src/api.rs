use crate::{
    node::{note::Note, project::Project, NodeType},
    BoardState,
};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct BoardTree {
    pub id: u64,
    pub node_type: NodeType,
    pub name: String,
    pub children: Vec<BoardTree>,
}

#[tauri::command]
pub fn add_directory(
    state: State<'_, BoardState>,
    name: &str,
    parent_id: u64,
) -> Result<(), String> {
    state
        .inner()
        .0
        .write()
        .unwrap()
        .add_new_directory(name, parent_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_note(state: State<'_, BoardState>, name: &str, parent_id: u64) -> Result<(), String> {
    state
        .inner()
        .0
        .write()
        .unwrap()
        .add_new_note(name, parent_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_project(state: State<'_, BoardState>, name: &str, parent_id: u64) -> Result<(), String> {
    state
        .inner()
        .0
        .write()
        .unwrap()
        .add_new_project(name, parent_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_note_content(
    state: State<'_, BoardState>,
    id: u64,
    content: &str,
) -> Result<(), String> {
    state
        .inner()
        .0
        .write()
        .unwrap()
        .set_note_content(id, content)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_node(
    state: State<'_, BoardState>,
    id: u64,
    parent_id: u64,
    recursive: bool,
) -> Result<(), String> {
    state
        .inner()
        .0
        .write()
        .unwrap()
        .delete_node(id, parent_id, recursive)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_node_type(state: State<'_, BoardState>, id: u64) -> Result<NodeType, String> {
    state
        .inner()
        .0
        .read()
        .unwrap()
        .get_node(id)
        .map(|node| node.get_type())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_note(state: State<'_, BoardState>, id: u64) -> Result<Note, String> {
    state
        .inner()
        .0
        .read()
        .unwrap()
        .get_node(id)
        .map_err(|e| e.to_string())?
        .as_any()
        .downcast_ref::<Note>()
        .ok_or(anyhow!("Node with id {id} is not a Note"))
        .cloned()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_project(state: State<'_, BoardState>, id: u64) -> Result<Project, String> {
    state
        .inner()
        .0
        .read()
        .unwrap()
        .get_node(id)
        .map_err(|e| e.to_string())?
        .as_any()
        .downcast_ref::<Project>()
        .ok_or(anyhow!("Node with id {id} is not a Project"))
        .cloned()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_board_tree(state: State<'_, BoardState>) -> Result<BoardTree, String> {
    Ok(state.inner().0.read().unwrap().as_board_tree())
}

#[tauri::command]
pub fn save_board(state: State<'_, BoardState>) -> Result<(), String> {
    state
        .inner()
        .0
        .write()
        .unwrap()
        .save()
        .map_err(|e| e.to_string())
}
