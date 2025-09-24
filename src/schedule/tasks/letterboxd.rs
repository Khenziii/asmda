use std::time::Duration;
use std::sync::Mutex;
use crate::schedule::tasks::{Task, TaskConfig};
use crate::{init_new_task};

fn callback() {
    println!("I should be running every second!");
}

init_new_task!(TaskConfig {
    callback: Box::new(callback),
    run_interval_seconds: 1,
});
