pub enum ArchiverIdentificator {
    Letterboxd,
}

pub enum APIWrapperIdentificator {
    Letterboxd,
    S3,
    Database,
}

impl ArchiverIdentificator {
    pub fn as_str(&self) -> &'static str {
        match self {
            ArchiverIdentificator::Letterboxd => "letterboxd",
        }
    }
}

impl APIWrapperIdentificator {
    pub fn as_str(&self) -> &'static str {
        match self {
            APIWrapperIdentificator::Letterboxd => "letterboxd",
            APIWrapperIdentificator::S3 => "s3",
            APIWrapperIdentificator::Database => "database",
        }
    }
}
