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

use crate::api::{add_directory, add_note, get_board_tree, save_board};

#[derive(Default)]
pub struct BoardState(RwLock<Board>);

fn main() {
    tauri::Builder::default()
        .manage(BoardState::default())
        .invoke_handler(tauri::generate_handler![
            add_directory,
            add_note,
            add_project,
            get_board_tree,
            save_board
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
