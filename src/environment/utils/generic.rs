use crate::environment::constants::RunningEnvironment;
use dirs;

pub fn as_boolean(value: String) -> bool {
    value
        .parse::<bool>()
        .unwrap_or_else(|_| panic!("Failed to cast {} into a boolean!", &value))
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
