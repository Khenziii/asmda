pub mod browser;

pub trait APIWrapper {
    fn get_name(&self) -> &str;
}
