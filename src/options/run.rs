use crate::logger::logger;
use crate::schedule::Scheduler;
use crate::utils::{startup::startup, terminal::setup_tui};
use super::CommandOption;
use std::thread;

async fn callback() {
    setup_tui();
    logger().log("Starting up...");
    startup();

    let mut scheduler = Scheduler::new(None);
    scheduler.run().await;

    thread::park();
}

pub fn get_option() -> CommandOption {
    CommandOption {
        string_identifiers: vec![String::from("run")],
        callback: Box::new(|| Box::pin(callback())),
        description: String::from("Starts the program. This option can be omitted (just `$ asmda` works fine too), but is still included as it looks more readable in some scenarios."),
    }
}
