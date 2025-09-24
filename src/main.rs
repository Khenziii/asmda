mod archivers;
mod api_wrappers;
mod environment;
mod utils;
mod schedule;

use schedule::Scheduler;
use std::thread;

#[tokio::main]
async fn main() {
    let mut scheduler = Scheduler::new();
    scheduler.run().await;

    thread::park();
}
