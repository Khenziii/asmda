pub mod table;
pub mod utils;
pub mod types;

use crate::utils::terminal::clear_previous_lines;
use utils::format_new_rows;
use types::NewRowCallback;
use once_cell::sync::OnceCell;
use std::sync::{Mutex, MutexGuard};

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

    fn get_height(&self) -> usize {
        self.rows.len()
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

    pub fn add_rows(&mut self, new_rows: Vec<String>) {
        let current_tui_height = self.get_height();

        let mut formatted_rows = format_new_rows(new_rows.clone());
        self.rows.append(&mut formatted_rows);

        let callbacks = self.clone().new_rows_callbacks;
        for new_rows_callback in callbacks {
            new_rows_callback(self, new_rows.clone());
        }

        self.rerender(current_tui_height);
    }

    pub fn add_row(&mut self, new_row: String) {
        let new_rows = vec![new_row];
        self.add_rows(new_rows);
    }

    pub fn add_new_row_callback(&mut self, new_row_callback: NewRowCallback) {
        self.new_rows_callbacks.push(new_row_callback);
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

        #[test]
        fn formats_new_lines_properly() {
            logger().log("first\n\n\nsecond");
            logger().log("third");
            logger().log("fourth");

            let output = logger().get_history_buffer_as_string();

            assert_eq!(output, "first\n\n\nsecond\nthird\nfourth")
        }
    }
}
