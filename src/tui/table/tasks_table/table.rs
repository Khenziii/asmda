use crate::tui::table::tasks_table::item::TasksTableItem;
use crate::tui::table::{Table, TableItem};
use crate::tui::utils::{format_new_rows, get_centered_cell_from_string};
use comfy_table::Table as ComfyTable;
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::{UTF8_SOLID_INNER_BORDERS, UTF8_ROUND_CORNERS};
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
        self.table.set_header(vec![
            get_centered_cell_from_string("ID"),
            get_centered_cell_from_string("Name"),
            get_centered_cell_from_string("Next run"),
        ]);
        self.table.load_preset(UTF8_FULL);
        self.table.apply_modifier(UTF8_ROUND_CORNERS);
        self.table.apply_modifier(UTF8_SOLID_INNER_BORDERS);
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

impl Default for TasksTable {
    fn default() -> Self {
        Self::new()
    }
}

impl TasksTable {
    pub fn new() -> Self {
        let table = ComfyTable::new();
        let mut new_instance = Self {
            items: HashMap::new(),
            table,
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
