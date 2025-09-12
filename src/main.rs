mod archivers;
mod api_wrappers;
mod environment;

use archivers::*;

#[tokio::main]
async fn main() {
    let letterboxd_archiver = archivers::letterboxd::LetterboxdArchiver {};
    println!("{}", letterboxd_archiver.get_name());
    letterboxd_archiver.get_data().await;
}
