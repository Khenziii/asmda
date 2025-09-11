use crate::api_wrappers::{APIWrapper};

pub struct LetterboxdAPIWrapper;

impl APIWrapper for LetterboxdAPIWrapper {
    fn get_name(&self) -> &str {
        "letterboxd"
    }
}

impl LetterboxdAPIWrapper {
    pub fn login(&self) {
        todo!("implement!");
    }
}
