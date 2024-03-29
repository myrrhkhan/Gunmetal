use crate::consts_and_errors::*;
use crate::get_vars;
use crate::settings_utils::*;
use std::collections::HashMap;
use std::fs::{self};
use std::io::Write;
#[allow(dead_code, unused_imports)]
use std::process::Command;

/// Command to add an environment variable.
/// ### Arguments:
/// - key (String)
/// - var_submission (String)
/// ### Returns
/// Either a success message or an error message
/// ### Types of Errors
/// - JSONParseError, when a settings value is not found in settings.json
/// - MakeDirError, when the program is unable to make a dedicated directory when making a new settings.json file
/// - MakeFileError, when the program is unable to create a file
/// - EmptySettingsError, when settings.json is empty and the program is awaiting the user to add all settings.
/// - ProfileOpenError, when the program is unable to open a shell profile
/// - WriteToFileError, when the program is unable to write to a file (usually the shell profile file)
#[tauri::command]
pub fn add_var(key: String, var_submission: String) -> Result<String, String> {
    // check if null or empty
    if !(var_submission.contains("\0") || var_submission.is_empty()) {
        // check if variable is already there, if so, return
        let duplicate: bool = check_if_var_duplicate(&key, &var_submission);
        if duplicate {
            return Ok(String::from(var_added_already!()));
        }
        // Try to append variable
        let result = append(&key, &var_submission);
        return result;
    } else {
        return Err(String::from(invalid_char!()));
    }
}

/// Checks if a variable being submitted already exists, returns boolean
/// ### Arguments
/// key: variable
/// var_submission: desired submissions
/// ### Returns:
/// boolean, true if duplicate, false if not
fn check_if_var_duplicate(key: &String, var_submission: &String) -> bool {
    let status: bool;

    let map: HashMap<String, Vec<String>> = get_vars::get_vars().unwrap();
    let entries_option: Option<&Vec<String>> = map.get(key);

    match entries_option {
        None => status = false,
        Some(entries) => {
            if entries.contains(&var_submission) {
                status = true;
            } else {
                status = false;
            }
        }
    }

    return status;
}

#[allow(dead_code)]
#[cfg(target_os = "windows")]
fn append(key: &String, var_submission: &String) -> Result<String, String> {
    let output = Command::new("SetX")
        .args([var_submission, key])
        .output()
        .map_err(|err| construct_err_msg!(cmd_fail_start!(), err.to_string()))?;

    return Ok(String::from(add_var_success!()));
}

#[allow(dead_code)]
#[cfg(target_os = "linux")]
fn append(key: &String, var_submission: &String) -> Result<String, String> {
    // make settings file if not already made, return any errors
    path_exists("/etc/Environment Variable Editor/", "settings.json", true)?;

    // get shell profile path from settings
    let shell_string = gather_setting(
        "/etc/Environment Variable Editor/settings.json",
        "shell_profile",
    )?;

    return write_to_file(shell_string, &key, &var_submission);
}

/// Appends the key and environment variable
/// ### Arguments:
/// - key
/// - var_submission
/// ### Returns:
/// A success string or an error message string
/// ### Errors when:
/// - cannot find home directory
#[cfg(target_os = "macos")]
fn append(key: &String, var_submission: &String) -> Result<String, String> {
    // establish path to settings directory

    // TODO clean this section up

    let path_to_dir = format!("{}/{}", home_dir!(), mac_config_path!());
    let path_to_settings = format!("{}/{}", &path_to_dir, "settings.json");

    // make settings file if not already made, return any errors
    path_exists(&path_to_dir.as_str(), "settings.json", true)?;

    // get shell profile path from settings
    let shell_string = gather_setting(&path_to_settings.as_str(), "shell_profile")?;

    return write_to_file(shell_string, &key, &var_submission);
}

/// Writes the environment variable to the shell profile
/// ### Arguments:
/// - shell_path_string: path to the shell profile setting
/// - key: variable key to be modified
/// - var_submission: variable to be added
/// ### Returns:
/// - String indicating status
fn write_to_file(
    shell_path: String,
    key: &String,
    var_submission: &String,
) -> Result<String, String> {
    // open file
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(&shell_path)
        // if there's an error, convert error into a string using macros and return
        .map_err(|err| construct_err_msg!(profile_err!(&shell_path), err.to_string()))?;

    // make string to add to end of file
    let export_cmd: String = format!("\nexport {}=\"{}\":${}", &key, &var_submission, &key);

    file.write(export_cmd.as_bytes()).map_err(|err| {
        construct_err_msg!(write_file_err!(&export_cmd, &shell_path), err.to_string())
    })?;

    // if this point is reached, return success string
    return Ok(String::from(add_var_success!()));
}
