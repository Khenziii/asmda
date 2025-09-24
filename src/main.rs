mod archivers;
mod api_wrappers;
mod environment;
mod utils;
mod schedule;

use schedule::Scheduler;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // let letterboxd_archiver = archivers::letterboxd::LetterboxdArchiver {};
    // println!("{}", letterboxd_archiver.get_identificator().as_str());
    // let data = letterboxd_archiver.get_data().await;
    //
    // let s3 = api_wrappers::s3::S3Client::new().await;
    // s3.upload(letterboxd_archiver.get_identificator().as_str(), "backup.zip", data).await;

    let mut scheduler = Scheduler::new();
    scheduler.run();

    thread::sleep(Duration::from_secs(5));
}
