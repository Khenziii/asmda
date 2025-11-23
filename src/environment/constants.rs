#[derive(PartialEq, Debug)]
pub enum RunningEnvironment {
    Development,
    Production,
}

// All supported environment variables.
#[derive(Clone, Debug)]
pub enum EnvironmentVariable {
    LetterboxdPassword,
    LetterboxdUsername,
    LetterboxdBackupFrequency,
    LetterboxdBackupEnable,
    S3Region,
    S3Url,
    S3BucketName,
    S3AccessKey,
    S3SecretKey,
    SecretsAreEncrypted,
    SecretsDecryptionKey,
    SecretsDecryptionKeyPassphrase,
}

impl EnvironmentVariable {
    // Environment variable's actual name.
    pub fn as_str(&self) -> String {
        let str = match self {
            Self::LetterboxdPassword => "LETTERBOXD_PASSWORD",
            Self::LetterboxdUsername => "LETTERBOXD_USERNAME",
            Self::LetterboxdBackupFrequency => "LETTERBOXD_BACKUP_FREQUENCY",
            Self::LetterboxdBackupEnable => "LETTERBOXD_BACKUP_ENABLE",
            Self::S3Region => "S3_REGION",
            Self::S3Url => "S3_URL",
            Self::S3BucketName => "S3_BUCKET_NAME",
            Self::S3AccessKey => "S3_ACCESS_KEY",
            Self::S3SecretKey => "S3_SECRET_KEY",
            Self::SecretsAreEncrypted => "SECRETS_ARE_ENCRYPTED",
            Self::SecretsDecryptionKey => "SECRETS_DECRYPTION_KEY",
            Self::SecretsDecryptionKeyPassphrase => "SECRETS_DECRYPTION_KEY_PASSPHRASE",
        };
        str.to_string()
    }

    // Whether they can be encrypted.
    pub fn can_be_encrypted(&self) -> bool {
        match self {
            Self::LetterboxdPassword => true,
            Self::LetterboxdUsername => false,
            Self::LetterboxdBackupFrequency => false,
            Self::LetterboxdBackupEnable => false,
            Self::S3Region => false,
            Self::S3Url => false,
            Self::S3BucketName => false,
            Self::S3AccessKey => false,
            Self::S3SecretKey => true,
            Self::SecretsAreEncrypted => false,
            Self::SecretsDecryptionKey => false,
            Self::SecretsDecryptionKeyPassphrase => false,
        }
    }

    pub fn get_development_fallback_value(&self) -> Option<String> {
        let value = match self {
            Self::LetterboxdPassword => None,
            Self::LetterboxdUsername => None,
            Self::LetterboxdBackupFrequency => Some("60"),
            Self::LetterboxdBackupEnable => Some("true"),
            Self::S3Region => Some("eu-central-1"),
            Self::S3Url => Some("http://localhost:9000"),
            Self::S3BucketName => Some("backups"),
            Self::S3AccessKey => Some("developmentuser"),
            Self::S3SecretKey => Some("developmentpassword"),
            Self::SecretsAreEncrypted => Some("false"),
            Self::SecretsDecryptionKey => None,
            Self::SecretsDecryptionKeyPassphrase => None,
        };
        value.map(|value| value.to_string())
    }

    // In production all environment variables are required to be explicitly defined.
    pub fn is_required(&self) -> bool {
        if self.get_development_fallback_value().is_some() {
            return false;
        }

        #[allow(clippy::match_like_matches_macro)]
        match self {
            Self::SecretsDecryptionKey | Self::SecretsDecryptionKeyPassphrase => false,
            _ => true,
        }
    }
}
