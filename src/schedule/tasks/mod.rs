mod letterboxd;
pub mod utils;

use crate::api_wrappers::database::DatabaseClient;
use crate::utils::constants::ArchiverIdentificator;
use std::time::{Duration, SystemTime};
use utils::types::ThreadCallback;

pub struct Task {
    next_run: SystemTime,
    interval: Duration,
    callback: ThreadCallback,
    app_name: ArchiverIdentificator,
    database: DatabaseClient,
}

impl Task {
    pub fn new(
        interval: Duration,
        callback: ThreadCallback,
        app_name: ArchiverIdentificator,
    ) -> Self {
        let database = DatabaseClient::new();
        let next_run = database.get_next_run_by_app_name(app_name.clone());

        Self {
            interval,
            callback,
            app_name,
            database,
            next_run,
        }
    }

    pub fn get_time_until_next_run(&self) -> Duration {
        let now = SystemTime::now();

        match self.next_run.duration_since(now) {
            Ok(duration) => duration,
            Err(_) => Duration::from_secs(0), // We're already past the date.
        }
    }

    pub async fn run(&mut self) {
        self.next_run += self.interval;
        self.database
            .update_next_run(self.app_name.clone(), self.next_run);

        let future = {
            let mut callback = self.callback.lock().expect("Failed to access callback!");
            (callback)()
        };
        future.await;
    }

    pub fn get_app_name(&self) -> ArchiverIdentificator {
        self.app_name.clone()
    }
}

pub fn get_all_tasks() -> Vec<Task> {
    vec![letterboxd::get_task()]
}
