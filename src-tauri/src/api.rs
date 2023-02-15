use crate::BoardState;

use tauri::State;

#[tauri::command]
pub fn get_project_names(state: State<'_, BoardState>) -> Result<Vec<Option<String>>, String> {
    let board = state.inner().0.read().unwrap();
    let children_ids = board.get_children_ids(0).map_err(|e| e.to_string())?;
    Ok(board.get_names_for_ids(&children_ids))
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
