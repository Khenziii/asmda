use crate::environment::constants::{EnvironmentVariable, RunningEnvironment};
use crate::environment::utils::decryption_key_passphrase::decryption_key_passphrase;
use crate::environment::utils::generic::{as_boolean, get_running_environment};
use crate::utils::encryption::EncryptionManager;
use crate::utils::multithreading;
use secrecy::ExposeSecret;

fn default_variable_value_parser(value: String) -> String {
    value.clone().replace("\\n", "\n")
}

pub trait EnvironmentVariableGetterResultParser {
    fn from_result(value: Option<String>, context: EnvironmentVariable) -> Self;
}

impl EnvironmentVariableGetterResultParser for String {
    fn from_result(value: Option<String>, context: EnvironmentVariable) -> Self {
        match value {
            Some(v) => default_variable_value_parser(v),
            None if context.is_required() => {
                panic!("Environment variable {} not set!", context.as_str())
            }
            None => String::from(""),
        }
    }
}

impl EnvironmentVariableGetterResultParser for Option<String> {
    fn from_result(value: Option<String>, _: EnvironmentVariable) -> Self {
        value.map(default_variable_value_parser)
    }
}

pub fn get_env_var_with_potential_fallback<T: EnvironmentVariableGetterResultParser>(
    variable: EnvironmentVariable,
) -> T {
    let running_environment = get_running_environment();
    let key = variable.as_str();
    let fallback = variable.get_development_fallback_value();

    let value: Option<String> =
        if running_environment == RunningEnvironment::Development && fallback.is_some() {
            Some(std::env::var(key.clone()).unwrap_or_else(|_| fallback.unwrap().to_string()))
        } else {
            std::env::var(&key).ok()
        };

    T::from_result(value, variable)
}

async fn get_env_var_async<T: EnvironmentVariableGetterResultParser>(
    variable: EnvironmentVariable,
) -> T {
    let using_encryption_str =
        get_env_var_with_potential_fallback(EnvironmentVariable::SecretsAreEncrypted);
    let using_encryption = as_boolean(using_encryption_str);

    let mut value = get_env_var_with_potential_fallback::<Option<String>>(variable.clone());
    let value_same_as_fallback = value == variable.get_development_fallback_value();

    if variable.can_be_encrypted() && using_encryption && value.is_some() && !value_same_as_fallback
    {
        let option_key: Option<String> =
            get_env_var_with_potential_fallback(EnvironmentVariable::SecretsDecryptionKey);
        let key = option_key.expect("Encryption key is not defined, even though `SECRETS_ARE_ENCRYPTED` is set to true. Please configure it and rerun the program");
        let key_passphrase = decryption_key_passphrase().clone().unwrap();

        let encryption_manager =
            EncryptionManager::new(key, key_passphrase.expose_secret().to_string()).await;
        let decrypted = encryption_manager.decrypt(value.unwrap()).await;
        value = Some(decrypted);
    }

    T::from_result(value, variable)
}

pub fn get_env_var<T: EnvironmentVariableGetterResultParser + Send + 'static>(
    variable: EnvironmentVariable,
) -> T {
    multithreading::block_on(get_env_var_async(variable))
}
