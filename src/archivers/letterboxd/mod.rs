use crate::archivers::{Archiver, InstantArchiver};

pub struct LetterboxdArchiver;

impl Archiver for LetterboxdArchiver {
    fn get_name(&self) -> &str {
        "letterboxd"
    }
}

impl InstantArchiver for LetterboxdArchiver {
    fn get_data(&self) -> Vec<u8> {

        todo!("download letterboxd data from https://letterboxd.com/data/export");
    }
}
