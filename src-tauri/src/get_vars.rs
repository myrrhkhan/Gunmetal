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

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for (key, vals) in std::env::vars() {
        let entries: Vec<String> = vals.split(":").map(str::to_string).collect();
        map.insert(key, entries);
    }

    // modify map by adding stuff from shell profile
    read_shell_profile(&shell_profile_path, &mut map)?;

    return Ok(map);
}

fn read_shell_profile(
    shell_profile_path: &String,
    map: &mut HashMap<String, Vec<String>>,
) -> Result<(), String> {
    // read through lines
    match read_lines(&shell_profile_path) {
        // if can read all the lines
        Ok(lines) => {
            // iterate through lines
            for line_buf in lines {
                // if can interpret line
                if let Ok(line) = line_buf {
                    // if line has export command, add to map
                    if &line.len() > &6 && &line[..6] == "export" {
                        // append_cmd_to_map(line, map)?;
                        match append_cmd_to_map(&line, map) {
                            Ok(()) => continue,
                            Err(_) => {
                                let x = &line.clone();
                                map.insert(String::from("Error:"), vec![x.to_string()]);
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            println!("{}", err.to_string());
            return Err(err.to_string());
        }
    }
    return Ok(());
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = fs::File::open(&filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn append_cmd_to_map(line: &String, map: &mut HashMap<String, Vec<String>>) -> Result<(), String> {
    // split the assignment (format key=value)
    let key_var_init = &line[7..].split("=").collect::<Vec<&str>>();
    let key = String::from(key_var_init[0]); // retrieve key
    let mut values_to_add: Vec<String> = Vec::new(); // values of key that we should add

    // value may have multiple values, ex: $PATH=a:b:c, take right half after =, split by : and convert to vec
    let value_entries_unfiltered = key_var_init[1]
        .replace("\"", "")
        .replace("\'", "")
        .split(":")
        .into_iter()
        .map(|entry| entry.to_string())
        .collect::<Vec<String>>();

    // parse variables and add to vector, see if we need to update the existing vector in the map
    let update_existing =
        parse_variables(line, &key, &value_entries_unfiltered, &mut values_to_add)?; // whether we update the existing variable with more values or we overwrite

    match map.get(&key) {
        Some(existing_values) => {
            if update_existing {
                let mut temp = values_to_add.clone();
                values_to_add = existing_values.to_owned();
                values_to_add.append(&mut temp);
            } else {
                println!("overriding, {}", &line);
            }
        }
        None => (),
    }

    map.insert(key, values_to_add);

    return Ok(());
}

fn parse_variables(
    line: &String,
    key: &String,
    value_entries_unfiltered: &Vec<String>,
    values_to_add: &mut Vec<String>,
) -> Result<bool, String> {
    // determines whether we're updating an existing variable or overriding
    let mut update_existing = false;
    if value_entries_unfiltered.len() != 1 {
        // iterate through all values of the key
        // if there's a reference to the original key, that means we're appending, update_existing = true
        for i in 0..(value_entries_unfiltered.len()) {
            let value_entry = &value_entries_unfiltered[i];

            // find reference to current key
            let first_char: char;
            let option_first_char = value_entry.chars().nth(0);
            match option_first_char {
                Some(ch) => first_char = ch,
                None => return Err(format!("Could not retrieve first char of line {}", &line)),
            }
            // if reference exists, don't add to list, but change update_existing
            if first_char == '$' {
                let (_, temp) = value_entry.split_at(1);
                if &String::from(temp) == key {
                    update_existing = true;
                    continue;
                }
            }

            values_to_add.push(value_entry.to_owned());
        }
    } else {
        values_to_add.push(value_entries_unfiltered[0].clone());
    }

    Ok(update_existing)
}
