use crate::tui::table::tasks_table::item::TasksTableItem;
use crate::tui::table::{Table, TableItem};
use crate::tui::utils::format_new_rows;
use comfy_table::Table as ComfyTable;
use std::collections::HashMap;

#[derive(Clone)]
pub struct TasksTable {
    items: HashMap<String, TasksTableItem>,
    table: ComfyTable,
}

impl Table<TasksTableItem> for TasksTable {
    fn get_height(&self) -> usize {
        let strings = self.as_string_array();
        strings.len()
    }

    fn as_string_array(&self) -> Vec<String> {
        let as_string = format!("{}", self.table);
        format_new_rows(vec![as_string])
    }

    // Resets the table to the base state.
    fn setup(&mut self) {
        let table = ComfyTable::new();
        self.table = table;
        self.table.set_header(["ID", "Name", "Next run"]);
    }

    fn add_item(&mut self, name: String, item: TasksTableItem) {
        self.items.insert(name, item);
        self.rerender();
    }

    fn rerender(&mut self) {
        self.setup();

        for (name, item) in &self.items {
            let mut row: Vec<String> = Vec::new();
            row.push(name.to_string());
            row.append(&mut item.get_value_as_string_array());
            self.table.add_row(row);
        }
    }
}

impl TasksTable {
    pub fn new() -> Self {
        let table = ComfyTable::new();
        let mut new_instance = Self {
            items: HashMap::new(),
            table: table,
        };
        new_instance.setup();
        new_instance
    }
}

#[cfg(test)]
mod tests {
    mod tasks_table {
        use super::super::*;

        #[test]
        fn calculates_height_correctly() {
            let mut table = TasksTable::new();
            table.add_item(
                "Test".to_string(),
                TasksTableItem {
                    name: "Task name".to_string(),
                    next_run: "Next run".to_string(),
                },
            );

            assert_eq!(table.get_height(), 5);
        }
    }
}
