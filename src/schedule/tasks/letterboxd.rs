use crate::api_wrappers::s3::S3Client;
use crate::api_wrappers::s3::utils::get_backup_path_for_archiver;
use crate::archivers::InstantArchiver;
use crate::archivers::letterboxd::LetterboxdArchiver;
use crate::environment::environment;
use crate::logger::logger;
use crate::schedule::tasks::Task;
use crate::schedule::tasks::utils::types::TaskConfig;
use crate::status::status_server;
use crate::utils::constants::ArchiverIdentificator;
use crate::{init_new_task, task_callback};
use std::sync::Mutex;
use std::time::Duration;

async fn callback() {
    let letterboxd_archiver = LetterboxdArchiver {};
    let data = letterboxd_archiver.get_data().await;

    if let Err(error) = data {
        let error_message = error.to_string();
        logger().error(&error_message);

        let new_status = Some("Failed to archive Letterboxd!".to_string());
        status_server().set_error_message(new_status);
    } else {
        let s3 = S3Client::new().await;
        s3.upload(
            &get_backup_path_for_archiver(letterboxd_archiver),
            "backup.zip",
            data.unwrap(),
        )
        .await;
    }
}

init_new_task!(TaskConfig {
    callback: task_callback!(callback),
    run_interval_seconds: environment().letterboxd.backup_frequency,
    app_name: ArchiverIdentificator::Letterboxd,
    is_enabled: environment().letterboxd.backup_enable,
});
