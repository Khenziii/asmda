pub use crate::environment::constants::RunningEnvironment;

pub struct Metadata {
    pub running_environment: RunningEnvironment,
    pub database_path: String,
}

pub struct LetterboxdEnvironment {
    pub password: String,
    pub username: String,
}

pub struct S3Environment {
    pub region: String,
    pub url: String,
    pub bucket_name: String,
    pub access_key: String,
    pub secret_key: String,
}

pub struct SecretsEnvironment {
    // Whether the raw values of the secrets (passed via environment variables) are encrypted or
    // not. Anything accessed via this program's `environment::environment` method will already
    // be decrypted for you.
    pub are_encrypted: bool,
}
pub struct Environment {
    // All additional data that is handy to have returned by the `environment` method, but isn't
    // configurable by environment variables.
    pub metadata: Metadata,
    pub letterboxd: LetterboxdEnvironment,
    pub s3: S3Environment,
    pub secrets: SecretsEnvironment,
}
