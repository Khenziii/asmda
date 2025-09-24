pub mod types;
pub mod utils;

use dotenv::dotenv;
use once_cell::sync::OnceCell;
use types::{
    Environment,
    LetterboxdEnvironment,
    S3Environment,
};
use utils::{
    get_running_environment,
    get_env_var,
    get_env_var_with_fallback,
};

static ENVIRONMENT: OnceCell<Environment> = OnceCell::new();

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
