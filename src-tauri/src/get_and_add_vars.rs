use crate::consts_and_errors::*;
use crate::settings_utils::*;
use dirs::home_dir;
use std::collections::HashMap;
use std::fs::{self};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
#[allow(dead_code, unused_imports)]
use std::process::Command;

#[tauri::command]
#[cfg(target_os = "windows")]
pub fn get_vars() -> Result<HashMap<String, Vec<String>>, String> {
    println!("calling again");
    // TODO: update so that it does not panic?

    // create map for variables and entries
    let mut names_and_vars: HashMap<String, Vec<String>> = HashMap::new();

    // procedure to save all keys and vals into map

    for (key, vals) in std::env::vars() {
        // convert string into vector by splitting, then map &str to String
        let entries: Vec<String> = vals.split(":").map(str::to_string).collect();
        names_and_vars.insert(key, entries);
    }

    return Ok(names_and_vars);
}

#[tauri::command]
#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn get_vars() -> Result<HashMap<String, Vec<String>>, String> {
    // find settings file
    let config_path: PathBuf = if std::env::consts::OS == "macos" {
        PathBuf::from(mac_config_path!())
    } else {
        PathBuf::from(linux_config_path!())
    };
    check_and_make_file(config_path, "settings.json")?; // return error if no settings file

    let config_path_string = config_path.to_str().unwrap();
    let shell_profile_path = gather_setting(
        format!("{}/settings.json", config_path_string).as_str(),
        "shell_profile",
    )?;

    // make map
    let mut names_and_vars: HashMap<String, Vec<String>> = HashMap::new();
    // add to map
    for (key, vals) in std::env::vars() {
        // convert string into vector by splitting, then map &str to String
        let entries: Vec<String> = vals.split(":").map(str::to_string).collect();
        names_and_vars.insert(key, entries);
    }
    // add shell file stuff to map as well
    read_path(&mut names_and_vars, &shell_profile_path);

    return Ok(names_and_vars);
}

fn read_path(map: &mut HashMap<String, Vec<String>>, shell_profile_path: &String) {
    if let Ok(lines) = read_lines(&shell_profile_path) {
        for line_buf in lines {
            if let Ok(line) = line_buf {
                if &line[..6] == "export" {
                    let key_var_init = &line[7..].split("=").collect::<Vec<&str>>();
                    let key = key_var_init[0];
                    let var = key_var_init[1].split(":").collect::<Vec<&str>>();
                    if var.len() == 1 {
                        let var_ref = find_env_refs(String::from(var[0]), &map);
                        let mut val_added: String = String::from("");
                        match var_ref {
                            Some(key) => {
                                val_added = var[0].replace(&key, map.get(&key).unwrap()[0].as_str())
                            }
                            None => val_added = String::from(var[0]),
                        }
                    }
                }
            }
        }
    }
}

// TODO: replace naive approach with regex
fn find_env_refs(var: String, map: &HashMap<String, Vec<String>>) -> Option<String> {
    let mut reference = "";
    let mut add_var = false;
    for letter in var.chars() {
        if letter == '$' {
            add_var = true;
        } else if add_var {
            reference = format!("{}{}", reference, letter).as_str();
        } else if letter == '/' {
            break;
        }
    }
    match reference {
        "" => None,
        _ => Some(String::from(reference)),
    }
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = fs::File::open(&filename)?;
    Ok(io::BufReader::new(file).lines())
}

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

    let map: HashMap<String, Vec<String>> = get_vars();
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
    check_and_make_file(
        PathBuf::from("/etc/Environment Variable Editor/"),
        "settings.json",
    )?;

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
    let mut path_to_dir: PathBuf = home_dir().unwrap();
    path_to_dir.push(mac_config_path!());
    let mut path_to_settings = home_dir().unwrap();
    path_to_settings.push(format!("{}/settings.json", mac_config_path!()));

    // make settings file if not already made, return any errors
    check_and_make_file(path_to_dir, "settings.json")?;

    // get shell profile path from settings
    let shell_string = gather_setting(path_to_settings.to_str().unwrap(), "shell_profile")?;

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
