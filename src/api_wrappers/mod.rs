pub mod browser;
pub mod s3;

pub trait APIWrapper {
    fn get_name(&self) -> &str;
}
