// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use settings_utils::{gather_setting, get_config_path};
mod add_vars;
mod consts_and_errors;
mod get_vars;
mod settings_utils;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_vars::get_vars,
            add_vars::add_var,
            get_shell_location
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_shell_location() -> Result<String, String> {
    let config_path = get_config_path();
    match config_path {
        Ok(path) => {
            match gather_setting(format!("{}/settings.json", path).as_str(), "shell_profile") {
                Ok(setting) => Ok(setting),
                Err(_) => Err(String::from("")), // error will already show up anyway, just ignore
            }
        }
        Err(_) => Err(String::from("")), // error will already show up anyway, just ignore
    }
}
