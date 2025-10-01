use asmda::schedule;
use asmda::schedule::tasks::Task;
use asmda::schedule::tasks::utils::types::TaskConfig;
use asmda::utils::constants;
use asmda::{init_new_task, task_callback};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;
use once_cell::sync::Lazy;

static FIRST_COUNTER: Lazy<Arc<Mutex<u32>>> = Lazy::new(|| Arc::new(Mutex::new(0)));
static SECOND_COUNTER: Lazy<Arc<Mutex<u32>>> = Lazy::new(|| Arc::new(Mutex::new(0)));

async fn first_task_callback() {
    let mut lock = FIRST_COUNTER.lock().unwrap();
    *lock += 1;
}

async fn second_task_callback() {
    let mut lock = SECOND_COUNTER.lock().unwrap();
    *lock += 1;
}

init_new_task!(
    TaskConfig {
        callback: task_callback!(first_task_callback),
        run_interval_seconds: 3,
        app_name: constants::ArchiverIdentificator::Tests,
    },
    get_first_task
);

init_new_task!(
    TaskConfig {
        callback: task_callback!(second_task_callback),
        run_interval_seconds: 4,
        app_name: constants::ArchiverIdentificator::Tests,
    },
    get_second_task
);

#[cfg(test)]
mod tests {
    mod scheduler {
        use super::super::*;

        #[tokio::test]
        async fn schedules_tasks() {
            let tasks = vec![get_first_task(), get_second_task()];
            let mut scheduler = schedule::Scheduler::new(Some(tasks));

            let scheduler_handle = tokio::spawn(async move {
                scheduler.run().await;
            });

            sleep(Duration::from_secs(10)).await;

            scheduler_handle.abort();

            // In 10 seconds:
            // - a task executing every 3 seconds should run 4 times,
            // - a task executing every 4 seconds should run 3 times.
            // The number of executions is increased by one because we divide execute it instantly
            // in the first iteration.
            let first_counter_value = FIRST_COUNTER.lock().unwrap().clone();
            let second_counter_value = SECOND_COUNTER.lock().unwrap().clone();
            assert_eq!(first_counter_value, 4);
            assert_eq!(second_counter_value, 3);
        }
    }
}
