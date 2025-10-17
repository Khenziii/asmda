use crate::tui::table::TableItem;

pub struct TasksTableItem {
    pub name: String,
    pub next_run: String,
}

impl TableItem for TasksTableItem {
    fn get_value_as_string_array(&self) -> Vec<String> {
        vec![self.name.clone(), self.next_run.clone()]
    }
}
