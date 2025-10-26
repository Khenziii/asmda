pub mod api_wrappers;
pub mod archivers;
pub mod environment;
pub mod logger;
pub mod schedule;
pub mod tui;
pub mod utils;

use logger::logger;
use schedule::Scheduler;
use std::thread;
use utils::{terminal::setup_tui, startup::startup};

pub async fn run() {
    setup_tui();
    logger().log("Starting up...");
    startup();

    let mut scheduler = Scheduler::new(None);
    scheduler.run().await;

    thread::park();
}
