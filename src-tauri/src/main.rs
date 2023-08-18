// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, process::Command};
mod add_vars;
mod consts_and_errors;
mod get_vars;
mod settings_utils;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_vars::get_vars,
            add_vars::add_var,
            test
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn test() -> String {
    let pwd = Command::new("pwd").output().unwrap();
    return String::from_utf8(pwd.stdout).unwrap();
}
