pub mod constants;
pub mod types;
pub mod utils;

use constants::EnvironmentVariable::*;
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use types::{
    Environment, LetterboxdEnvironment, Metadata, S3Environment, SecretsEnvironment,
    StatusServerEnvironment, WebDriverEnvironment,
};
use utils::decryption_key_passphrase::decryption_key_passphrase;
use utils::environment::get_env_var;
use utils::generic::{
    as_boolean, as_integer, get_database_path, get_logs_directory_path, get_program_version,
    get_running_environment,
};

static ENVIRONMENT: OnceCell<Environment> = OnceCell::new();

pub fn environment() -> &'static Environment {
    ENVIRONMENT.get_or_init(|| {
        dotenv().ok();
        Environment {
            metadata: Metadata {
                database_path: get_database_path(),
                running_environment: get_running_environment(),
                logs_directory_path: get_logs_directory_path(),
                program_version: get_program_version(),
            },
            letterboxd: LetterboxdEnvironment {
                password: get_env_var(LetterboxdPassword),
                username: get_env_var(LetterboxdUsername),
                backup_frequency: as_integer(get_env_var(LetterboxdBackupFrequency)),
                backup_enable: as_boolean(get_env_var(LetterboxdBackupEnable)),
            },
            s3: S3Environment {
                region: get_env_var(S3Region),
                url: get_env_var(S3Url),
                bucket_name: get_env_var(S3BucketName),
                access_key: get_env_var(S3AccessKey),
                secret_key: get_env_var(S3SecretKey),
            },
            secrets: SecretsEnvironment {
                are_encrypted: as_boolean(get_env_var(SecretsAreEncrypted)),
                decryption_key: get_env_var(SecretsDecryptionKey),
                decryption_key_passphrase: decryption_key_passphrase().clone(),
            },
            status_server: StatusServerEnvironment {
                enable: as_boolean(get_env_var(StatusServerEnable)),
                port: as_integer(get_env_var(StatusServerPort)),
            },
            webdriver: WebDriverEnvironment {
                url: get_env_var(WebDriverUrl),
                port: as_integer(get_env_var(WebDriverPort)),
            },
        }
    })
}
