use crate::schedule::tasks::{Task, get_all_tasks};
use crate::tui::table::tasks_table::utils::{
    add_tasks_to_tasks_table, get_tasks_table_height_by_tasks,
};
use crate::tui::table::{Table, tasks_table::table::TasksTable};
use crate::tui::{self, TerminalUserInterface};
use crate::utils::terminal::{println, refresh_table_in_tui};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct ThreadSafeTaskData {
    pub name: String,
    pub next_run: Duration,
}

impl ThreadSafeTaskData {
    pub fn from_task(task: Task) -> Self {
        Self {
            name: task.get_app_name().as_str(),
            next_run: task.get_time_until_next_run(),
        }
    }
}

pub fn convert_tasks_to_thread_safe_task_data(tasks: Vec<Task>) -> Vec<ThreadSafeTaskData> {
    let mut thread_safe_task_data = Vec::new();

    for task in tasks {
        let data = ThreadSafeTaskData::from_task(task);
        thread_safe_task_data.push(data);
    }

    thread_safe_task_data
}

fn refresh_tasks_table(
    tasks: Vec<ThreadSafeTaskData>,
    table: &mut TasksTable,
    tui: &mut TerminalUserInterface,
) {
    table.reinitialize();
    add_tasks_to_tasks_table(tasks, table);
    refresh_table_in_tui(table.clone(), tui);
}

// In the first iteration, we need to manually add padding for the table, as our future
// code expects that previous render of table is in place and will try to override last
// `table_height` lines.
fn add_initial_table_padding(table: Arc<Mutex<TasksTable>>) {
    let tasks = get_all_tasks();
    let thread_safe_task_data = convert_tasks_to_thread_safe_task_data(tasks);
    let mut locked_table = table.lock().unwrap();
    let table_future_height =
        get_tasks_table_height_by_tasks(thread_safe_task_data.clone(), &mut locked_table);

    for _ in 0..table_future_height {
        println("");
    }
}

pub type TableTasksDataGetter = Option<Arc<Box<dyn Fn() -> Vec<ThreadSafeTaskData> + Send + Sync>>>;

pub fn setup_tasks_table_in_tui(
    table: Arc<Mutex<TasksTable>>,
    get_tasks_data: TableTasksDataGetter,
) {
    add_initial_table_padding(table.clone());

    thread::spawn(move || {
        loop {
            {
                let mut tui = tui::tui();
                let mut locked_table = table.lock().unwrap();
                let thread_safe_tasks_data = match get_tasks_data.clone() {
                    Some(f) => f(),
                    None => {
                        let tasks = get_all_tasks();
                        convert_tasks_to_thread_safe_task_data(tasks)
                    }
                };

                refresh_tasks_table(thread_safe_tasks_data.clone(), &mut locked_table, &mut tui);
            }

            thread::sleep(Duration::from_secs(1));
        }
    });
}
