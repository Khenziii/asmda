use crate::utils::uuid;

fn tests_identificator_as_str() -> String {
    format!("tests_{}", &uuid::get_random())
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
            ArchiverIdentificator::Tests => tests_identificator_as_str(),
            ArchiverIdentificator::Letterboxd => "letterboxd".to_string(),
        }
    }
}

impl APIWrapperIdentificator {
    pub fn as_str(&self) -> String {
        match self {
            APIWrapperIdentificator::Tests => tests_identificator_as_str(),
            APIWrapperIdentificator::Letterboxd => "letterboxd".to_string(),
            APIWrapperIdentificator::S3 => "s3".to_string(),
            APIWrapperIdentificator::Database => "database".to_string(),
        }
    }
}

pub static LOCAL_POLLING_RATE_MS: u64 = 250;
