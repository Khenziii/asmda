pub mod browser;
pub mod s3;
pub mod database;

use crate::utils::constants::APIWrapperIdentificator;

pub trait APIWrapper {
    fn get_identificator(&self) -> APIWrapperIdentificator;
}
