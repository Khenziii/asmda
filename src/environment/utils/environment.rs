use crate::environment::constants::{EnvironmentVariable, RunningEnvironment};
use crate::environment::utils::generic::{get_running_environment, as_boolean};
use crate::utils::encryption::Decryptor;

pub trait EnvironmentVariableGetterResultParser {
    fn from_result(value: Option<String>, context: EnvironmentVariable) -> Self;
}

impl EnvironmentVariableGetterResultParser for String {
    fn from_result(value: Option<String>, context: EnvironmentVariable) -> Self {
        match value {
            Some(v) => v,
            None if context.is_required() => panic!("Environment variable {} not set!", context.as_str()),
            None => String::from(""),
        }
    }
}

impl EnvironmentVariableGetterResultParser for Option<String> {
    fn from_result(value: Option<String>, _: EnvironmentVariable) -> Self {
        value
    }
}

fn get_env_var_by_with_potential_fallback<T: EnvironmentVariableGetterResultParser>(variable: EnvironmentVariable) -> T {
    let running_environment = get_running_environment();
    let key = variable.as_str();
    let fallback = variable.get_development_fallback_value();

    let value: Option<String> = if running_environment == RunningEnvironment::Development && fallback.is_some() {
        Some(std::env::var(key.clone()).unwrap_or_else(|_| fallback.unwrap().to_string()))
    } else {
        std::env::var(&key).ok()
    };

    T::from_result(value, variable)
}

pub fn get_env_var<T: EnvironmentVariableGetterResultParser>(variable: EnvironmentVariable) -> T {
    let using_encryption_str = get_env_var_by_with_potential_fallback(EnvironmentVariable::SecretsAreEncrypted);
    let using_encryption = as_boolean(using_encryption_str);

    let mut value = get_env_var_by_with_potential_fallback::<Option<String>>(variable.clone());

    if variable.can_be_encrypted() && using_encryption && value.is_some() {
        let decryptor = Decryptor::new_sync("todo".to_string(), "todo".to_string());
        let decrypted = decryptor.decrypt_sync(value.unwrap());
        value = Some(decrypted);
    }

    T::from_result(value, variable)
}
