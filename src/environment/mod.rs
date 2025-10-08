pub mod constants;
pub mod types;
pub mod utils;

use constants::EnvironmentVariable::*;
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use types::{Environment, LetterboxdEnvironment, Metadata, S3Environment, SecretsEnvironment};
use utils::{as_boolean, get_database_path, get_env_var, get_running_environment};

static ENVIRONMENT: OnceCell<Environment> = OnceCell::new();

pub fn environment() -> &'static Environment {
    ENVIRONMENT.get_or_init(|| {
        dotenv().ok();
        Environment {
            metadata: Metadata {
                database_path: get_database_path(),
                running_environment: get_running_environment(),
            },
            letterboxd: LetterboxdEnvironment {
                password: get_env_var(LetterboxdPassword),
                username: get_env_var(LetterboxdUsername),
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
            },
        }
    })
}
