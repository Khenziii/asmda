#[derive(PartialEq)]
pub enum RunningEnvironment {
    Development,
    Production,
}

// All supported environment variables.
pub enum EnvironmentVariable {
    LetterboxdPassword,
    LetterboxdUsername,
    S3Region,
    S3Url,
    S3BucketName,
    S3AccessKey,
    S3SecretKey,
    SecretsAreEncrypted,
}

impl EnvironmentVariable {
    // Environment variable's actual name.
    pub fn as_str(&self) -> String {
        let str = match self {
            Self::LetterboxdPassword => "LETTERBOXD_PASSWORD",
            Self::LetterboxdUsername => "LETTERBOXD_USERNAME",
            Self::S3Region => "S3_REGION",
            Self::S3Url => "S3_URL",
            Self::S3BucketName => "S3_BUCKET_NAME",
            Self::S3AccessKey => "S3_ACCESS_KEY",
            Self::S3SecretKey => "S3_SECRET_KEY",
            Self::SecretsAreEncrypted => "SECRETS_ARE_ENCRYPTED",
        };
        str.to_string()
    }

    // Whether they can be encrypted.
    pub fn can_be_encrypted(&self) -> bool {
        match self {
            Self::LetterboxdPassword => true,
            Self::LetterboxdUsername => false,
            Self::S3Region => false,
            Self::S3Url => false,
            Self::S3BucketName => false,
            Self::S3AccessKey => false,
            Self::S3SecretKey => true,
            Self::SecretsAreEncrypted => false,
        }
    }

    pub fn get_fallback_value(&self) -> Option<String> {
        let value = match self {
            Self::LetterboxdPassword => None,
            Self::LetterboxdUsername => None,
            Self::S3Region => Some("eu-central-1"),
            Self::S3Url => Some("http://localhost:9000"),
            Self::S3BucketName => Some("backups"),
            Self::S3AccessKey => Some("developmentuser"),
            Self::S3SecretKey => Some("developmentpassword"),
            Self::SecretsAreEncrypted => Some("false"),
        };

        match value {
            Some(value) => Some(value.to_string()),
            None => None,
        }
    }
}
