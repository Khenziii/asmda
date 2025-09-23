use dotenv::dotenv;
use once_cell::sync::OnceCell;

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
    pub letterboxd: LetterboxdEnvironment,
    pub s3: S3Environment,
    pub running_environment: RunningEnvironment,
}

static ENVIRONMENT: OnceCell<Environment> = OnceCell::new();

fn get_running_environment() -> RunningEnvironment {
    if cfg!(debug_assertions) { return RunningEnvironment::Development; }
    RunningEnvironment::Production
}

fn get_env_var(key: &str) -> String {
    std::env::var(key).expect(&format!("Environment variable {} not set", key))
}

fn get_env_var_with_fallback(key: &str, fallback: &str) -> String {
    let running_environment = get_running_environment();

    if running_environment == RunningEnvironment::Development {
        return std::env::var(key).unwrap_or_else(|_| fallback.to_string());
    }

    get_env_var(key)
}

pub fn environment() -> &'static Environment {
    ENVIRONMENT.get_or_init(|| {
        dotenv().ok();
        Environment {
            running_environment: get_running_environment(),
            letterboxd: LetterboxdEnvironment {
                password: get_env_var("LETTERBOXD_PASSWORD"),
                username: get_env_var("LETTERBOXD_USERNAME"),
            },
            s3: S3Environment {
                region: get_env_var_with_fallback("S3_REGION", "eu-central-1"),
                url: get_env_var_with_fallback("S3_URL", "http://localhost:9000"),
                bucket_name: get_env_var_with_fallback("S3_BUCKET_NAME", "backups"),
                access_key: get_env_var_with_fallback("S3_SECRET_KEY", "developmentuser"),
                secret_key: get_env_var_with_fallback("S3_ACCESS_KEY", "developmentpassword"),
            },
        }
    })
}
