use crate::environment::constants::{EnvironmentVariable, RunningEnvironment};
use crate::utils::encryption::Decryptor;
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

pub fn get_env_var_by_name(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("Environment variable {} not set", key))
}

pub fn get_env_var_by_name_with_fallback(key: &str, fallback: &str) -> String {
    let running_environment = get_running_environment();

    if running_environment == RunningEnvironment::Development {
        return std::env::var(key).unwrap_or_else(|_| fallback.to_string());
    }

    get_env_var_by_name(key)
}

pub fn get_env_var(variable: EnvironmentVariable) -> String {
    let using_encryption_str = get_env_var_by_name_with_fallback(
        &EnvironmentVariable::SecretsAreEncrypted.as_str(),
        &EnvironmentVariable::SecretsAreEncrypted
            .get_fallback_value()
            .unwrap(),
    );
    let using_encryption = as_boolean(using_encryption_str);

    let mut value = match variable.get_fallback_value() {
        Some(fallback) => get_env_var_by_name_with_fallback(&variable.as_str(), &fallback),
        None => get_env_var_by_name(&variable.as_str()),
    };

    if variable.can_be_encrypted() && using_encryption {
        let decryptor = Decryptor::new_sync("todo".to_string(), "todo".to_string());
        let decrypted = decryptor.decrypt_sync(value);
        value = decrypted;
    }

    value
}
