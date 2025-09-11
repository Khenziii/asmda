mod archivers;
mod api_wrappers;

use archivers::*;
// use api_wrappers::*;

use crate::api_wrappers::browser::{BrowserAPIWrapper};

#[tokio::main]
async fn main() {
    let browser = BrowserAPIWrapper {};
    browser.launch().await;

    let letterboxd_archiver = archivers::letterboxd::LetterboxdArchiver {};
    println!("{}", letterboxd_archiver.get_name());
}
