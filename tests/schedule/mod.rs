use asmda::logger;
use asmda::schedule;
use asmda::schedule::tasks::Task;
use asmda::schedule::tasks::utils::types::TaskConfig;
use asmda::utils::constants;
use asmda::{init_new_task, task_callback};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex as TokioMutex;
use tokio::time::sleep;

static FIRST_COUNTER: Lazy<Arc<Mutex<u32>>> = Lazy::new(|| Arc::new(Mutex::new(0)));
static SECOND_COUNTER: Lazy<Arc<Mutex<u32>>> = Lazy::new(|| Arc::new(Mutex::new(0)));
static THIRD_TIMER: Lazy<Arc<Mutex<SystemTime>>> = Lazy::new(|| Arc::new(SystemTime::now().into()));

async fn first_task_callback() {
    logger::debug("First task running!");

    let mut lock = FIRST_COUNTER.lock().unwrap();
    *lock += 1;
}

async fn second_task_callback() {
    logger::debug("Second task running!");

    let mut lock = SECOND_COUNTER.lock().unwrap();
    *lock += 1;
}

async fn third_task_callback() {
    logger::debug("Third task running!");

    let mut lock = THIRD_TIMER.lock().unwrap();
    *lock = SystemTime::now();
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

init_new_task!(
    TaskConfig {
        callback: task_callback!(third_task_callback),
        run_interval_seconds: 5,
        app_name: constants::ArchiverIdentificator::Tests,
    },
    get_third_task
);

#[cfg(test)]
mod tests {
    mod scheduler {
        use std::time::UNIX_EPOCH;

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

        #[tokio::test]
        async fn remembers_tasks_scheduled_execution_date_across_restarts() {
            let tasks = vec![get_third_task()];
            let scheduler = Arc::new(TokioMutex::new(schedule::Scheduler::new(Some(tasks))));
            let start_timestamp = SystemTime::now();

            // Start the Scheduler for 2 seconds.
            let scheduler_clone = scheduler.clone();
            let scheduler_handle = tokio::spawn(async move {
                let mut unwrapped_scheduler = scheduler_clone.lock().await;
                unwrapped_scheduler.run().await;
            });
            sleep(Duration::from_secs(2)).await;
            scheduler_handle.abort();

            // Let it run again, this time for 7 seconds.
            let scheduler_handle = tokio::spawn(async move {
                let mut unwrapped_scheduler = scheduler.lock().await;
                unwrapped_scheduler.run().await;
            });
            sleep(Duration::from_secs(7)).await;
            scheduler_handle.abort();

            // Our task was meant to be executed 5 seconds after the `start_timestamp`, no matter
            // whether the process restarted or not. Verify that latest execution time is equal to
            // `start_timestamp` + 5 seconds.
            let latest_execution_timestamp = THIRD_TIMER.lock().unwrap().clone();
            let latest_timestamps_seconds = latest_execution_timestamp
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let adjusted_start_timestamp = start_timestamp + Duration::from_secs(5);
            let adjusted_start_timestamp_seconds = adjusted_start_timestamp
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            assert_eq!(latest_timestamps_seconds, adjusted_start_timestamp_seconds,);
        }
    }
}
