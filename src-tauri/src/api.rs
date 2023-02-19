use crate::{node::NodeType, BoardState};

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
pub fn get_board_tree(state: State<'_, BoardState>) -> Result<BoardTree, String> {
    Ok(state.inner().0.read().unwrap().as_board_tree())
}
