use crate::consts_and_errors::*;
use crate::settings_utils::*;
use std::collections::HashMap;
use std::fs::{self};
use std::io::{self, BufRead};
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
    let config_path = if std::env::consts::OS == "macos" {
        format!("{}/{}", home_dir!(), mac_config_path!())
    } else {
        format!("{}/{}", home_dir!(), linux_config_path!())
    };
    path_exists(&config_path.as_str(), "settings.json", true)?; // return error if no settings file

    // get the path to the shell profile
    let shell_profile_path = gather_setting(
        format!("{}/settings.json", config_path).as_str(),
        "shell_profile",
    )?;
    // check if shell profile path exists, if not return error
    path_exists_combined_path(&shell_profile_path, false)?;

    // read the shell profile
    match read_shell_profile(&shell_profile_path) {
        Ok(names_and_vars) => return Ok(names_and_vars),
        Err(err) => {
            println!("returning {}", err.to_string());
            return Err(err);
        }
    }
}

fn read_shell_profile(shell_profile_path: &String) -> Result<HashMap<String, Vec<String>>, String> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    // read through lines
    match read_lines(&shell_profile_path) {
        // if can read all the lines
        Ok(lines) => {
            // iterate through lines
            for line_buf in lines {
                println!("here's a linebuf");
                // if can interpret line
                if let Ok(line) = line_buf {
                    println!("{}", line);
                    // if line has export command, add to map
                    if &line.len() > &6 && &line[..6] == "export" {
                        println!("{}", line);
                        append_cmd_to_map(line, &mut map);
                    }
                }
            }
        }
        Err(err) => {
            println!("{}", err.to_string());
            return Err(err.to_string());
        }
    }
    return Ok(map);
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = fs::File::open(&filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn append_cmd_to_map(line: String, map: &mut HashMap<String, Vec<String>>) {
    // split the assignment (format key=value)
    let key_var_init = &line[7..].split("=").collect::<Vec<&str>>();
    let key = String::from(key_var_init[0]); // retrieve key

    // value may have multiple values, ex: $PATH=a:b:c, take right half after =, split by : and convert to vec
    let mut var_entries = key_var_init[1]
        .split(":")
        .into_iter()
        .map(|entry| entry.to_string())
        .collect::<Vec<String>>();

    // if no split (i.e. one single key)
    if var_entries.len() == 1 {
        // analyze key, find references to other variables
        var_entries[0] = simplify_key(&var_entries[0].clone(), map);
        // add variable to hashmap
        map.insert(key, var_entries);
    } else {
        // iterate through all multiple values
        // iterate through entries, if entry does not equal the key, simplify key
        for i in 0..(var_entries.len()) {
            let entry = &var_entries[i];
            let mut end = "";
            if entry.chars().nth(0).unwrap() == '$' {
                let (_, temp) = entry.split_at(0);
                end = temp;
            }
            // can unwrap b/c length is guaranteed to be not empty
            if end != &key {
                var_entries[i] = simplify_key(&entry, map);
            } else {
                var_entries.remove(i);
            }
        }

        match map.get(&key) {
            None => map.insert(key, var_entries),
            Some(existing_vals) => {
                let vals = existing_vals.clone();
                for entry in var_entries {
                    vals.push(entry);
                }
                map.insert(key, vals);
                // Ok(());
            }
        }
    }
    return;
}

fn simplify_key(entry: &String, map: &HashMap<String, Vec<String>>) -> String {
    // find references to other variables by searching for $
    let reference_in_var = find_env_refs(entry);
    // if reference found, replace
    let var = entry;
    match reference_in_var {
        Some(key) => {
            let key_replacement = map.get(&key).unwrap()[0].as_str();
            let _ = var.replace(&key, key_replacement);
        }
        _ => (),
    }
    return var.to_string();
}

// TODO: replace naive approach with regex
fn find_env_refs(var: &String) -> Option<String> {
    let mut reference = String::from("");
    let mut add_var = false;
    for letter in var.chars() {
        if letter == '$' {
            add_var = true;
        } else if add_var {
            reference = format!("{}{}", reference, letter);
        } else if letter == '/' {
            break;
        }
    }
    match reference.as_str() {
        "" => None,
        _ => Some(String::from(reference)),
    }
}
