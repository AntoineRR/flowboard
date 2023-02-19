#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod api;
mod board;
mod node;
mod utils;

use std::sync::RwLock;

use api::add_project;
use board::Board;

use crate::api::get_board_tree;

#[derive(Default)]
pub struct BoardState(RwLock<Board>);

fn main() {
    tauri::Builder::default()
        .manage(BoardState::default())
        .invoke_handler(tauri::generate_handler![add_project, get_board_tree])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
