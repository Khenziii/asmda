pub mod api_wrappers;
pub mod archivers;
pub mod environment;
pub mod logger;
pub mod schedule;
pub mod utils;
pub mod tui;

use schedule::Scheduler;
use std::thread;
use utils::startup::startup;
use logger::logger;

pub async fn run() {
    logger().log("Starting up...");
    startup();

    let mut scheduler = Scheduler::new(None);
    scheduler.run().await;

    thread::park();
}
