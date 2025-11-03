use crate::tui::table::Table;
use crate::tui::table::tasks_table::{item::TasksTableItem, table::TasksTable};
use crate::tui::table::utils::ThreadSafeTaskData;

pub fn add_tasks_to_tasks_table(tasks: Vec<ThreadSafeTaskData>, table: &mut TasksTable) {
    let mut id = 1;
    for task in tasks {
        table.add_item(
            id.to_string(),
            TasksTableItem {
                name: task.name,
                next_run: format!("{}s", task.next_run.as_secs()),
            },
        );
        id += 1;
    }
}

pub fn get_tasks_table_height_by_tasks(
    tasks: Vec<ThreadSafeTaskData>,
    table: &mut TasksTable,
) -> usize {
    table.reinitialize();
    add_tasks_to_tasks_table(tasks, table);
    table.get_height()
}
