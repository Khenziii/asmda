use crate::environment::types::RunningEnvironment;
use dirs;

pub fn get_running_environment() -> RunningEnvironment {
    if cfg!(debug_assertions) {
        return RunningEnvironment::Development;
    }
    RunningEnvironment::Production
}

pub fn get_env_var(key: &str) -> String {
    std::env::var(key).expect(&format!("Environment variable {} not set", key))
}

pub fn get_env_var_with_fallback(key: &str, fallback: &str) -> String {
    let running_environment = get_running_environment();

    if running_environment == RunningEnvironment::Development {
        return std::env::var(key).unwrap_or_else(|_| fallback.to_string());
    }

    get_env_var(key)
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
