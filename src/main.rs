mod archivers;
mod api_wrappers;
mod environment;

use archivers::*;
// use api_wrappers::*;

use crate::api_wrappers::browser::letterboxd::{LetterboxdBrowserAPIWrapper};

#[tokio::main]
async fn main() {
    let browser = LetterboxdBrowserAPIWrapper {};
    browser.launch().await;

    let letterboxd_archiver = archivers::letterboxd::LetterboxdArchiver {};
    println!("{}", letterboxd_archiver.get_name());
}
