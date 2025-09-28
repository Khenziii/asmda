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

#[derive(PartialEq)]
pub enum RunningEnvironment {
    Development,
    Production,
}

pub struct Environment {
    pub database_path: String,
    pub letterboxd: LetterboxdEnvironment,
    pub s3: S3Environment,
    pub running_environment: RunningEnvironment,
}
