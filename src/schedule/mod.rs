pub mod tasks;

use crate::logger::logger;
use std::mem::take;
use tasks::{Task, get_all_tasks};

pub struct Scheduler {
    tasks: Vec<Task>,
}

impl Scheduler {
    // Defaults to supporting all tasks.
    pub fn new(tasks_arg: Option<Vec<Task>>) -> Self {
        let tasks = match tasks_arg {
            Some(tasks_arg) => tasks_arg,
            None => get_all_tasks(),
        };

        Self { tasks }
    }

    pub async fn run(&mut self) {
        let tasks = take(&mut self.tasks);
        for mut task in tasks {
            tokio::spawn(async move {
                loop {
                    let time_until_next_run = task.get_time_until_next_run();
                    let app_name = task.get_app_name();

                    tokio::time::sleep(time_until_next_run).await;

                    logger().log(&format!(
                        "Archiving {}...",
                        app_name.as_str(),
                    ));

                    task.run().await;

                    logger().log(&format!(
                        "Finished archiving {}!",
                        app_name.as_str(),
                    ));
                }
            });
        }
    }
}
