macro_rules! home_dir {
    () => {
        dirs::home_dir()
            .expect("Can't find home dir")
            .to_str()
            .expect("Can't change home dir to string")
    };
}

macro_rules! mac_config_path {
    () => {
        ".config/Environment Variable Editor"
    };
}

macro_rules! linux_config_path {
    () => {
        "/etc/Environment Variable Editor"
    };
}

macro_rules! construct_err_msg {
    ($message:expr, $full_error:expr) => {
        format!("{}\nFull Error:\n{}", $message, $full_error)
    };
}

macro_rules! mkdir_err {
    ($dir:expr) => {
        format!(
          "Could not make a directory at {}. Please see the help page and follow instructions on making a settings directory.",
          $dir
        )
    };
}
macro_rules! make_file_err {
    ($path:expr) => {
        format!(
          "Could not make desired file at {}. Please see the help page and follow the steps to diagnose.",
          $path
        )
    };
}
macro_rules! profile_err {
    ($path:expr) => {
        format!(
          "Could not open shell profile ({}). Please check that the shell profile setting points to the right file (either is either in {} or {}) and try again.",
          $path,
          mac_config_path!(),
          linux_config_path!()
        )
    };
}
macro_rules! settings_read_error {
  ($path:expr) => {
    format!(
      "Could not find or read from settings file at {}. Please make sure the file exists, run the program as sudo/admin, and try again",
      $path
    )
  }
}
macro_rules! write_file_err {
    ($content:expr, $path:expr) => {
        format!(
            "Could not write the message \"{}\" to file {}. Please write manually",
            $content, $path
        )
    };
}
// the following macros don't take arguments, but i'm still using macros for the sake of consistency anyway
macro_rules! json_parse_err {
    () => {
      "Value not found in settings file. Please open the settings page and ensure that all settings are set."
    };
}
macro_rules! empty_settings_err {
  () => {
    "Settings file is empty. Please fill out all settings in settings.json before trying again.\nsettings.json is either in \"/etc/Environment Variable Editor/\" or \"~/.config/Environment Variable Editor/\""
  };
}
macro_rules! add_var_success {
    () => {
      "Variable added successfully!\n\nThe new variable will not show up in this window unless the app is closed and open again, but the variable should be there."
    };
}
macro_rules! var_added_already {
    () => {
        "Variable has been added already."
    };
}
macro_rules! invalid_char {
    () => {
        "Invalid input, contains null character or is empty."
    };
}

#[allow(unused_macros)]
macro_rules! cmd_fail_start {
    () => {
        "Command failed to run"
    };
}

pub(crate) use add_var_success;
#[allow(unused_imports)]
pub(crate) use cmd_fail_start;
pub(crate) use construct_err_msg;
pub(crate) use empty_settings_err;
pub(crate) use home_dir;
pub(crate) use invalid_char;
pub(crate) use json_parse_err;
pub(crate) use linux_config_path;
pub(crate) use mac_config_path;
pub(crate) use make_file_err;
pub(crate) use mkdir_err;
pub(crate) use profile_err;
pub(crate) use settings_read_error;
pub(crate) use var_added_already;
pub(crate) use write_file_err;
