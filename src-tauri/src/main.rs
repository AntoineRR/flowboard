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

use crate::api::{
    add_directory, add_note, delete_node, get_board_tree, get_node_type, get_note, get_project,
    save_board, set_note_content,
};

pub struct BoardState(RwLock<Board>);

impl Default for BoardState {
    fn default() -> Self {
        Self(RwLock::new(Board::load_or_create()))
    }
}

fn main() {
    tauri::Builder::default()
        .manage(BoardState::default())
        .invoke_handler(tauri::generate_handler![
            add_directory,
            add_note,
            add_project,
            delete_node,
            set_note_content,
            get_node_type,
            get_note,
            get_project,
            get_board_tree,
            save_board
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
