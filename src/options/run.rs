use crate::init_command_option;
use crate::logger::logger;
use crate::schedule::Scheduler;
use crate::utils::{startup::startup, terminal::setup_tui};
use super::CommandOption;
use std::thread;

async fn callback() {
    setup_tui(None);
    logger().log("Starting up...");
    startup();

    let mut scheduler = Scheduler::new(None);
    scheduler.run().await;

    thread::park();
}

init_command_option!(
    vec!["run"],
    "Starts the program. This option can be omitted (just `$ asmda` works fine too), but is still included as it looks more readable in some scenarios.",
    callback
);
