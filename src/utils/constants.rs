use std::time::{SystemTime, UNIX_EPOCH};

fn get_current_time_string() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string()
}

#[derive(Clone)]
pub enum ArchiverIdentificator {
    Tests,
    Letterboxd,
}

#[derive(Clone)]
pub enum APIWrapperIdentificator {
    Tests,
    Letterboxd,
    S3,
    Database,
}

impl ArchiverIdentificator {
    pub fn as_str(&self) -> String {
        match self {
            ArchiverIdentificator::Tests => format!("tests_{}", &get_current_time_string()),
            ArchiverIdentificator::Letterboxd => "letterboxd".to_string(),
        }
    }
}

impl APIWrapperIdentificator {
    pub fn as_str(&self) -> String {
        match self {
            APIWrapperIdentificator::Tests => "tests".to_string(),
            APIWrapperIdentificator::Letterboxd => "letterboxd".to_string(),
            APIWrapperIdentificator::S3 => "s3".to_string(),
            APIWrapperIdentificator::Database => "database".to_string(),
        }
    }
}
