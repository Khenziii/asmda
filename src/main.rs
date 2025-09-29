mod api_wrappers;
mod archivers;
mod environment;
mod logger;
mod schedule;
mod utils;

use schedule::Scheduler;
use std::thread;
use utils::startup::startup;

#[tokio::main]
async fn main() {
    logger::log("Starting up...");
    startup();

    let mut scheduler = Scheduler::new();
    scheduler.run().await;

    thread::park();
}
