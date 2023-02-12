use crate::BoardState;

use tauri::State;

#[tauri::command]
pub fn get_project_names(state: State<'_, BoardState>) -> Vec<String> {
    state.inner().0.read().unwrap().get_project_names()
}

#[tauri::command]
pub fn add_project(state: State<'_, BoardState>, name: &str) {
    state.inner().0.write().unwrap().add_new_project(name);
}
