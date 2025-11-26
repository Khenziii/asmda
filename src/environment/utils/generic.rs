use crate::environment::constants::RunningEnvironment;
use dirs;

pub fn as_boolean(value: String) -> bool {
    value
        .parse::<bool>()
        .unwrap_or_else(|_| panic!("Failed to cast {} into a boolean!", &value))
}

pub fn as_integer(value: String) -> u64 {
    value
        .parse::<u64>()
        .unwrap_or_else(|_| panic!("Failed to cast {} into an u64!", &value))
}

pub fn get_running_environment() -> RunningEnvironment {
    if cfg!(debug_assertions) {
        return RunningEnvironment::Development;
    }
    RunningEnvironment::Production
}

pub fn get_database_path() -> String {
    let running_environment = get_running_environment();

    if running_environment == RunningEnvironment::Production {
        dirs::data_dir()
            .expect("Failed to get the data dir!")
            .join("asmda.sqlite")
            .to_str()
            .expect("Failed to convert to str!")
            .to_string()
    } else {
        String::from("asmda.sqlite")
    }
}

pub fn get_logs_directory_path() -> String {
    let running_environment = get_running_environment();

    if running_environment == RunningEnvironment::Production {
        let state_dir = dirs::state_dir()
            .expect("Failed to get the state dir!")
            .to_str()
            .unwrap()
            .to_string();
        let apps_logs_state_dir = format!("{}/asmda/logs", state_dir);
        apps_logs_state_dir
    } else {
        String::from("./logs")
    }
}

pub fn get_program_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
