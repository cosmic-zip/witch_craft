// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod core;
pub mod meow;
use crate::core::core::*;
use crate::meow::meow::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![select_report])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
