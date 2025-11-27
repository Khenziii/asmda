use crate::environment;
use crate::utils::time::get_current_path_friendly_formatted_date;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io, os};

// For example: "~/.local/state/asmda/logs".
fn get_logs_directory_path() -> PathBuf {
    let config = environment::environment();
    let raw_path = config.metadata.logs_directory_path.clone();
    PathBuf::from(&raw_path)
}

// For example: "~/.local/state/asmda", used for `latest.log` symlink in the root folder of program's
// data dir, for readability purposes.
fn get_logs_directory_parent_path() -> PathBuf {
    let logs_directory_path = get_logs_directory_path().canonicalize().unwrap();
    logs_directory_path
        .parent()
        .expect("Failed to get logs directory parent! It might not be available.")
        .to_path_buf()
}

pub fn get_latest_log_symlink_path() -> PathBuf {
    let logs_directory_parent_path = get_logs_directory_parent_path();
    let symlink_path_raw = format!(
        "{}/latest.log",
        logs_directory_parent_path.to_str().unwrap()
    );
    PathBuf::from(symlink_path_raw)
}

fn create_symlink(original_path: &Path, link_path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        os::unix::fs::symlink(original_path, link_path)
    }

    #[cfg(windows)]
    {
        if original_path.is_dir() {
            os::windows::fs::symlink_dir(original_path, link_path)
        } else {
            os::windows::fs::symlink_file(original_path, link_path)
        }
    }
}

pub fn validate_log_directory_setup() {
    let logs_directory_path = get_logs_directory_path();

    let current_formatted_date = get_current_path_friendly_formatted_date();
    let new_log_filename = format!("{}.log", current_formatted_date);
    let new_log_file_path = format!(
        "{}/{}",
        logs_directory_path.to_str().unwrap(),
        new_log_filename
    );
    fs::File::create(new_log_file_path.clone()).expect("Failed to create log file path! Logs won't be accessible. Make sure that correct permissions are set.");

    let latest_log_symlink_path = get_latest_log_symlink_path();
    fs::remove_file(latest_log_symlink_path.clone()).ok();

    create_symlink(&PathBuf::from(new_log_file_path), &latest_log_symlink_path)
        .unwrap_or_else(|_| panic!("Failed to create latest log symlink! ({}) Logs at the standardized location *will* be missing.", latest_log_symlink_path.to_str().unwrap()));
}

pub fn set_logs_to_string_array(new_logs: Vec<String>) {
    let latest_log_symlink_path = get_latest_log_symlink_path();
    let mut latest_log_symlink =
        fs::File::create(latest_log_symlink_path).expect("Failed to open latest log file symlink!");

    for log in new_logs {
        writeln!(latest_log_symlink, "{}", log).expect("Failed to write logs to the file!");
    }
}
