pub mod tasks_table;

pub trait Table<T> {
    fn get_height(&self) -> usize;
    fn as_string_array(&self) -> Vec<String>;
    fn setup(&mut self);
    fn add_item(&mut self, name: String, item: T);
    fn rerender(&mut self);
}

pub trait TableItem {
    fn get_value_as_string_array(&self) -> Vec<String>;
}
