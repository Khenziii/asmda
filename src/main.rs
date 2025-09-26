mod archivers;
mod api_wrappers;
mod environment;
mod utils;
mod schedule;

use schedule::Scheduler;
use std::thread;
use utils::startup::startup;

#[tokio::main]
async fn main() {
    startup();

    let mut scheduler = Scheduler::new();
    scheduler.run().await;

    thread::park();
}
