use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::PathBuf,
    process::Command,
};

use crate::consts_and_errors::*;

#[derive(Deserialize, Serialize)]
struct SettingsFields {
    shell_profile: String,
}

pub fn path_exists_combined_path(path: &str, make_file: bool) -> Result<(), String> {
    let split_path = path.split("/").collect::<Vec<&str>>();
    match split_path.len() {
        1 => return path_exists("", path, make_file),
        _ => {
            let filename = split_path[split_path.len() - 1];
            let mut path_to_dir = String::from("");
            for i in 0..(split_path.len() - 1) {
                path_to_dir = format!("{}/{}", &path_to_dir, &split_path[i]);
            }
            return path_exists(path_to_dir.as_str(), filename, make_file);
        }
    }
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
pub fn path_exists(path_to_dir: &str, filename: &str, make_file: bool) -> Result<(), String> {
    // Add filename to make full string
    let mut full_path = PathBuf::from(&path_to_dir);
    full_path.push(filename);

    println!("{}", &full_path.to_str().unwrap());

    if full_path.exists() {
        return Ok(());
    } else if make_file == false {
        // if we don't wanna make file, return error saying file doesn't exist
        // convert file name to string. if error, return error
        let pathstr_option = &full_path.to_str();
        match pathstr_option {
            Some(pathstr) => return Err(format!("File {} does not exist.", pathstr)),
            None => return Err(String::from("File does not exist. Error extracting the file's name, so filename might not be readable"))
        }
    }

    // from here, we're making the file

    // check if directory exists, if not make the directory
    if !PathBuf::from(&path_to_dir).is_dir() {
        Command::new("mkdir")
            .args(["-p", &path_to_dir])
            .output()
            .map_err(|err| construct_err_msg!(mkdir_err!(&path_to_dir), err.to_string()))?;
    }
    // generate the JSON and return error empty file
    match &full_path.to_str() {
        Some(pathstr) => {
            fs::File::create(&full_path)
                .map_err(|err| construct_err_msg!(make_file_err!(pathstr), err.to_string()))?;
            generate_json(full_path.to_str().unwrap())?;
            return Err(String::from(empty_settings_err!()));
        }
        None => {
            return Err(String::from(
                "Could not make a file. Could not convert path to file into a readable string.",
            ))
        }
    }
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

// Returns the path of the configuration file
pub fn get_config_path() -> Result<String, String> {
    let config_path = if std::env::consts::OS == "macos" {
        Ok(format!("{}/{}", home_dir!(), mac_config_path!()))
    } else if std::env::consts::OS == "linux" {
        Ok(format!("{}/{}", home_dir!(), linux_config_path!()))
    } else {
        Err(String::from("Using Windows, no shell profile used."))
    };
    return config_path;
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
