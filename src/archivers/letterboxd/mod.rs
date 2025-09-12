use crate::archivers::{Archiver, InstantArchiver};
use crate::api_wrappers::browser::letterboxd::LetterboxdBrowserAPIWrapper;

pub struct LetterboxdArchiver;

impl Archiver for LetterboxdArchiver {
    fn get_name(&self) -> &str {
        "letterboxd"
    }
}

impl InstantArchiver for LetterboxdArchiver {
    async fn get_data(&self) -> Vec<u8> {
        let lettterboxd_wrapper = LetterboxdBrowserAPIWrapper::new().await;
        lettterboxd_wrapper.launch().await;
        lettterboxd_wrapper.export_data().await;
        lettterboxd_wrapper.close().await;

        todo!("Return the file here.");
    }
}
