mod letterboxd;

use std::sync::Mutex;
use std::time::{Duration, Instant};

type Callback = Mutex<Box<dyn FnMut() + Send>>;

pub struct Task {
    next_run: Instant,
    interval: Duration,
    callback: Callback,
}

impl Task {
    pub fn new(interval: Duration, callback: Callback) -> Self {
        Self {
            interval,
            callback,
            next_run: Instant::now(),
        }
    }

    pub fn get_time_until_next_run(&self) -> Duration {
        let now = Instant::now();

        let time_until_next_run = if self.next_run > now {
            self.next_run - now
        } else {
            Duration::from_secs(0)
        };
        time_until_next_run
    }

    pub fn run(&mut self) {
        self.next_run += self.interval;

        let mut callback = self.callback
            .lock()
            .expect("Failed to access callback!");
        (*callback)();
    }
}

pub struct TaskConfig {
    run_interval_seconds: u64,
    callback: Box<dyn FnMut() + Send>,
}

#[macro_export]
macro_rules! init_new_task {
    ($config:expr) => {
        pub fn get_task() -> Task {
            Task::new(
                Duration::from_secs($config.run_interval_seconds),
                Mutex::new($config.callback)
            )
        }
    }
}

pub fn get_tasks() -> Vec<Task> {
    let mut tasks = Vec::new();

    tasks.push(letterboxd::get_task());

    tasks
}
