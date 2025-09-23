mod archivers;
mod api_wrappers;
mod environment;

use archivers::*;

#[tokio::main]
async fn main() {
    let letterboxd_archiver = archivers::letterboxd::LetterboxdArchiver {};
    println!("{}", letterboxd_archiver.get_name());
    let data = letterboxd_archiver.get_data().await;

    let s3 = api_wrappers::s3::S3Client::new().await;
    s3.upload(letterboxd_archiver.get_name(), "backup.zip", data).await;
}
