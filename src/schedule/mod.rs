mod tasks;

use tokio;
use std::mem::take;
use tasks::{Task, get_tasks};
use crate::logger;

pub struct Scheduler {
    tasks: Vec<Task>,
}

impl Scheduler {
    pub fn new() -> Self {
        let tasks = get_tasks();

        Self { tasks }
    }

    pub async fn run(&mut self) {
        let tasks = take(&mut self.tasks);
        for mut task in tasks {
            tokio::spawn(async move {
                loop {
                    let time_until_next_run = task.get_time_until_next_run();

                    logger::debug(&format!("next archive in {} seconds", time_until_next_run.as_secs()));

                    tokio::time::sleep(time_until_next_run).await;

                    task.run().await;
                }
            });
        }
    }
}
