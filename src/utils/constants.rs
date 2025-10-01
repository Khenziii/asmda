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
    pub fn as_str(&self) -> &'static str {
        match self {
            ArchiverIdentificator::Tests => "tests",
            ArchiverIdentificator::Letterboxd => "letterboxd",
        }
    }
}

impl APIWrapperIdentificator {
    pub fn as_str(&self) -> &'static str {
        match self {
            APIWrapperIdentificator::Tests => "tests",
            APIWrapperIdentificator::Letterboxd => "letterboxd",
            APIWrapperIdentificator::S3 => "s3",
            APIWrapperIdentificator::Database => "database",
        }
    }
}
