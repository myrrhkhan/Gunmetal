use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::{Path, PathBuf},
    process::Command,
};

use crate::consts_and_errors::*;

#[derive(Deserialize, Serialize)]
struct SettingsFields {
    shell_profile: String,
}

/// Check if a file exists, and if not, make one
/// ### Arguments:
/// - path_to_dir: path to directory (do not include "/")
/// - filename: name of file
/// ### Returns
/// Either an empty string if the file exists or was successfully made, or an error message as a String
/// ### Panics or Unwraps
/// - when trying to convert an os string into a string
/// - when converting the pathbuf into a string
pub fn check_and_make_file(mut path_to_dir: PathBuf, filename: &str) -> Result<(), String> {
    // check if directory exists, if not make the directory
    if !&path_to_dir.is_dir() {
        // make a path to keep track of working directory
        let mut current_dir: String = String::from("");

        // iterate through folders and make them
        for folder in &path_to_dir {
            // add current folder to working directory
            current_dir = current_dir + "/" + folder.to_str().unwrap();

            // if directory doesn't exist, make it
            if !Path::new(&current_dir).exists() {
                Command::new("mkdir")
                    .args([&current_dir])
                    .output()
                    .map_err(|err| construct_err_msg!(mkdir_err!(current_dir), err.to_string()))?;
                // convert error to string and return
            }
        }
    }

    // Add filename to make full string
    path_to_dir.push(filename);

    // if a file doesn't exist, make the file or return the error
    if !&path_to_dir.exists() {
        fs::File::create(&path_to_dir).map_err(|err| {
            construct_err_msg!(
                make_file_err!(&path_to_dir.clone().into_os_string().into_string().unwrap()),
                err.to_string()
            )
        })?;
        generate_json(&path_to_dir.to_str().unwrap())?;
        return Err(String::from(empty_settings_err!()));
    }

    return Ok(());
}

/// Generates the JSON file with blank fields
fn generate_json(settings_path: &str) -> Result<(), String> {
    // initialize with empty fields
    let settings = SettingsFields {
        shell_profile: String::from(""),
    };

    // convert to JSON string
    let json_string = serde_json::to_string(&settings).unwrap();
    // write JSON string to file
    let write_result = fs::write(&settings_path, json_string).map_err(|err| err.to_string());

    return write_result;
}

/// Reads the JSON settings file, finds the value for a setting, and returns it
/// This program re-reads the JSON file every time a setting is needed in case the file is edited during runtime
/// ### Arguments
/// - settings_path: path to JSON file
/// - key: the setting that should be gathered
/// ### Returns:
/// Either a String with the setting or an error
pub fn gather_setting(settings_path: &str, key: &str) -> Result<String, String> {
    // read JSON to string, return error
    let settings_text: String = fs::read_to_string(&settings_path)
        .map_err(|err| construct_err_msg!(settings_read_error!(&settings_path), err.to_string()))?;
    // Parse JSON string, return the resulting string or return error string
    // TODO use key
    println!("{}", &settings_text);
    let settings_status: Result<SettingsFields, serde_json::Error> =
        serde_json::from_str(&settings_text);

    match settings_status {
        Ok(settings) => return Ok(settings.shell_profile),
        Err(error) => return Err(construct_err_msg!(json_parse_err!(), error.to_string())),
    }
}
