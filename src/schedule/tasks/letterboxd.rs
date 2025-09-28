use std::time::Duration;
use std::sync::Mutex;
use crate::schedule::tasks::{Task, TaskConfig};
use crate::archivers::{Archiver, InstantArchiver};
use crate::archivers::letterboxd::LetterboxdArchiver;
use crate::api_wrappers::s3::S3Client;
use crate::utils::constants::ArchiverIdentificator;
use crate::{init_new_task, task_callback};

async fn callback() {
    let letterboxd_archiver = LetterboxdArchiver {};
    let data = letterboxd_archiver.get_data().await;

    let s3 = S3Client::new().await;
    s3.upload(letterboxd_archiver.get_identificator().as_str(), "backup.zip", data).await;
}

init_new_task!(TaskConfig {
    callback: task_callback!(callback),
    run_interval_seconds: 60,
    app_name: ArchiverIdentificator::Letterboxd,
});
