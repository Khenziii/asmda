use crate::api_wrappers::s3::S3Client;
use crate::archivers::letterboxd::LetterboxdArchiver;
use crate::archivers::{Archiver, InstantArchiver};
use crate::schedule::tasks::Task;
use crate::schedule::tasks::utils::types::TaskConfig;
use crate::utils::constants::ArchiverIdentificator;
use crate::{init_new_task, task_callback};
use std::sync::Mutex;
use std::time::Duration;

async fn callback() {
    let letterboxd_archiver = LetterboxdArchiver {};
    let data = letterboxd_archiver.get_data().await;

    let s3 = S3Client::new().await;
    s3.upload(
        &letterboxd_archiver.get_identificator().as_str(),
        "backup.zip",
        data,
    )
    .await;
}

init_new_task!(TaskConfig {
    callback: task_callback!(callback),
    run_interval_seconds: 60,
    app_name: ArchiverIdentificator::Letterboxd,
});
