mod letterboxd;

use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::time::{Duration, Instant};

type Callback = Box<dyn FnMut() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;
type ThreadCallback = Mutex<Callback>;

pub struct Task {
    next_run: Instant,
    interval: Duration,
    callback: ThreadCallback,
}

impl Task {
    pub fn new(interval: Duration, callback: ThreadCallback) -> Self {
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

    pub async fn run(&mut self) {
        self.next_run += self.interval;

        let future = {
            let mut callback = self.callback
                .lock()
                .expect("Failed to access callback!");
            (callback)()
        };
        future.await;
    }
}

pub struct TaskConfig {
    run_interval_seconds: u64,
    callback: Callback,
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

#[macro_export]
macro_rules! task_callback {
    ($func:path) => {
        Box::new(|| Box::pin($func()))
    };
}

pub fn get_tasks() -> Vec<Task> {
    let mut tasks = Vec::new();

    tasks.push(letterboxd::get_task());

    tasks
}
