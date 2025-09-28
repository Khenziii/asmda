mod archivers;
mod api_wrappers;
mod environment;
mod utils;
mod schedule;
mod logger;

use std::thread;
use schedule::Scheduler;
use utils::startup::startup;

#[tokio::main]
async fn main() {
    logger::log("Starting up...");
    startup();

    let mut scheduler = Scheduler::new();
    scheduler.run().await;

    thread::park();
}
