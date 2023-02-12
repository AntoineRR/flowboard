#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod api;
mod board;
mod project;

use std::sync::RwLock;

use api::{add_project, get_project_names};
use board::Board;

#[derive(Default)]
pub struct BoardState(RwLock<Board>);

fn main() {
    tauri::Builder::default()
        .manage(BoardState::default())
        .invoke_handler(tauri::generate_handler![add_project, get_project_names])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
