use serde::{Deserialize, Serialize};
use std::{
    env::current_dir,
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
pub fn check_and_make_file(path_to_dir: &str, filename: &str) -> Result<(), String> {
    // check if directory exists, if not make the directory
    if !PathBuf::from(&path_to_dir).is_dir() {
        Command::new("mkdir")
            .args(["-p", &path_to_dir])
            .output()
            .map_err(|err| construct_err_msg!(mkdir_err!(&path_to_dir), err.to_string()))?;
    }

    // Add filename to make full string
    let mut full_path = PathBuf::from(&path_to_dir);
    full_path.push(filename);

    // if a file doesn't exist, make the file or return the error
    if !full_path.exists() {
        fs::File::create(&full_path).map_err(|err| {
            construct_err_msg!(
                make_file_err!(&full_path.clone().into_os_string().into_string().unwrap()),
                err.to_string()
            )
        })?;
        generate_json(full_path.to_str().unwrap())?;
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
