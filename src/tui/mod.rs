pub mod table;
pub mod types;
pub mod utils;

use crate::utils::terminal::clear_previous_lines;
use once_cell::sync::OnceCell;
use std::sync::{Mutex, MutexGuard};
use types::NewRowCallback;
use utils::format_new_rows;

#[derive(Clone)]
pub struct TerminalUserInterface {
    rows: Vec<String>,
    new_rows_callbacks: Vec<NewRowCallback>,
}

impl TerminalUserInterface {
    pub fn new() -> Self {
        TerminalUserInterface {
            rows: Vec::new(),
            new_rows_callbacks: Vec::new(),
        }
    }

    fn print(&self) {
        for row in &self.rows {
            println!("{}", row);
        }
    }

    fn rerender(&self, previous_height: usize) {
        let new_height = self.get_height();
        let height_difference = new_height - previous_height;

        for _ in 0..height_difference {
            print!("\n");
        }

        clear_previous_lines(new_height, None);
        self.print();
    }

    pub fn get_height(&self) -> usize {
        self.rows.len()
    }

    pub fn add_rows(&mut self, new_rows: Vec<String>, trigger_callbacks: bool, render: bool) {
        let current_tui_height = self.get_height();

        let formatted_rows = format_new_rows(new_rows.clone());
        self.rows.append(&mut formatted_rows.clone());

        if trigger_callbacks {
            let callbacks = self.clone().new_rows_callbacks;
            for new_rows_callback in callbacks {
                new_rows_callback(self, formatted_rows.clone());
            }
        }

        if render {
            self.rerender(current_tui_height);
        }
    }

    pub fn add_row(&mut self, new_row: String, trigger_callbacks: bool, render: bool) {
        let new_rows = vec![new_row];
        self.add_rows(new_rows, trigger_callbacks, render);
    }

    pub fn add_new_rows_callback(&mut self, new_row_callback: NewRowCallback) {
        self.new_rows_callbacks.push(new_row_callback);
    }

    pub fn remove_last_row(&mut self) {
        let current_amount_of_rows = self.get_height();
        if current_amount_of_rows > 0 {
            self.rows.truncate(current_amount_of_rows - 1);
        } else {
            self.rows.clear();
        }
    }

    pub fn remove_last_rows(&mut self, amount: usize) {
        for _ in 0..amount {
            self.remove_last_row();
        }
    }

    pub fn get_rows(&self) -> Vec<String> {
        self.rows.clone()
    }

    pub fn reinitialize(&mut self) {
        *self = TerminalUserInterface::new();
    }
}

static TUI: OnceCell<Mutex<TerminalUserInterface>> = OnceCell::new();

fn get_tui() -> &'static Mutex<TerminalUserInterface> {
    TUI.get_or_init(|| Mutex::new(TerminalUserInterface::new()))
}

pub fn tui() -> MutexGuard<'static, TerminalUserInterface> {
    get_tui().lock().unwrap()
}

#[cfg(test)]
mod tests {
    mod tui {
        use crate::logger::logger;
        use crate::utils::terminal::strip_color_from_string;

        #[test]
        fn formats_new_lines_properly() {
            logger().reinitialize();
            logger().log("first\n\n\nsecond");
            logger().log("third");
            logger().log("fourth");

            let output = strip_color_from_string(logger().get_history_buffer_as_string());
            assert_eq!(output, "first\n\n\nsecond\nthird\nfourth")
        }
    }
}
