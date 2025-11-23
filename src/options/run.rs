use super::CommandOption;
use crate::init_command_option;
use crate::logger::logger;
use crate::schedule::tasks::get_enabled_tasks;
use crate::schedule::Scheduler;
use crate::tui::table::utils::convert_tasks_to_thread_safe_task_data;
use crate::utils::{startup::startup, terminal::setup_tui};
use std::{thread, sync::Arc};

async fn callback() {
    setup_tui(Some(Arc::new(Box::new(|| {
        let tasks = get_enabled_tasks();
        convert_tasks_to_thread_safe_task_data(tasks)
    }))));
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
