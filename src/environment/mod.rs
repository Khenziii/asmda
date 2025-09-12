use dotenv::dotenv;
use once_cell::sync::OnceCell;

pub struct LetterboxdEnvironment {
    pub password: String,
    pub username: String,
}

pub struct Environment {
    pub letterboxd: LetterboxdEnvironment,
}

static ENVIRONMENT: OnceCell<Environment> = OnceCell::new();

fn get_env_var(key: &str) -> String {
    std::env::var(key).expect(&format!("Environment variable {} not set", key))
}

pub fn environment() -> &'static Environment {
    ENVIRONMENT.get_or_init(|| {
        dotenv().ok();
        Environment {
            letterboxd: LetterboxdEnvironment {
                password: get_env_var("LETTERBOXD_PASSWORD"),
                username: get_env_var("LETTERBOXD_USERNAME"),
            }
        }
    })
}
