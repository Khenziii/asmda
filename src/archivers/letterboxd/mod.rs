use crate::archivers::{Archiver, InstantArchiver};
use crate::api_wrappers::browser::letterboxd::LetterboxdBrowserAPIWrapper;
use crate::utils::constants::ArchiverIdentificator;

pub struct LetterboxdArchiver;

impl Archiver for LetterboxdArchiver {
    fn get_identificator(&self) -> ArchiverIdentificator {
        ArchiverIdentificator::Letterboxd
    }
}

impl InstantArchiver for LetterboxdArchiver {
    async fn get_data(&self) -> Vec<u8> {
        let lettterboxd_wrapper = LetterboxdBrowserAPIWrapper::new().await;
        lettterboxd_wrapper.launch().await;
        let data = lettterboxd_wrapper.export_data().await;
        lettterboxd_wrapper.close().await;

        data
    }
}
