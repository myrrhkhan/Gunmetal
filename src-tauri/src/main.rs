// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env};
mod get_and_add_vars;
mod errors;
mod settings_utils;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_and_add_vars::get_vars, 
      get_and_add_vars::add_var
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}