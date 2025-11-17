pub mod table;
pub mod types;
pub mod utils;

use crate::utils::logs::{set_logs_to_string_array, validate_log_directory_setup};
use crate::utils::terminal::{clear_previous_lines, println, strip_color_from_strings};
use once_cell::sync::OnceCell;
use std::sync::{Mutex, MutexGuard};
use types::NewRowCallback;
use utils::format_new_rows;

#[derive(Clone)]
pub struct TerminalUserInterface {
    rows: Vec<String>,
    new_rows_callbacks: Vec<NewRowCallback>,
    // Whether the table should keep updating. This is turned off for example when the program is
    // suspended, and we don't want anything writing to stanard output.
    is_active: bool,
    sync_to_log_file_on_update: bool,
}

impl Default for TerminalUserInterface {
    fn default() -> Self {
        Self::new(false)
    }
}

impl TerminalUserInterface {
    pub fn new(sync_to_log_file_on_update: bool) -> Self {
        if sync_to_log_file_on_update {
            validate_log_directory_setup();
        }

        TerminalUserInterface {
            rows: Vec::new(),
            new_rows_callbacks: Vec::new(),
            is_active: true,
            sync_to_log_file_on_update,
        }
    }

    fn print(&self) {
        for row in &self.rows {
            println(row);
        }
    }

    pub fn set_is_active(&mut self, new_is_active: bool) {
        self.is_active = new_is_active;
    }

    // If the total height of the TUI has changed, you'll need to pass `previous_height` in order
    // to keep everything synchronised. If it's the same, passing just `None` is completely fine.
    pub fn rerender(&self, previous_height: Option<usize>) {
        if !self.is_active { return };

        let current_height = self.get_height();
        if let Some(previous_height_raw) = previous_height {
            let height_difference = current_height - previous_height_raw;

            for _ in 0..height_difference {
                println("");
            }
        }

        clear_previous_lines(current_height, None);
        self.print();

        if self.sync_to_log_file_on_update {
            // Write current TUI state to log file.
            set_logs_to_string_array(strip_color_from_strings(self.rows.clone()));
        }
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
            self.rerender(Some(current_tui_height));
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
        *self = TerminalUserInterface::new(self.sync_to_log_file_on_update);
    }
}

static TUI: OnceCell<Mutex<TerminalUserInterface>> = OnceCell::new();

fn get_tui() -> &'static Mutex<TerminalUserInterface> {
    TUI.get_or_init(|| Mutex::new(TerminalUserInterface::new(true)))
}

pub fn tui() -> MutexGuard<'static, TerminalUserInterface> {
    get_tui().lock().unwrap()
}

#[cfg(test)]
mod tests {
    mod tui {
        use crate::logger::logger;
        use crate::utils::terminal::strip_color_from_string;
        use serial_test::serial;

        #[test]
        #[serial]
        fn formats_new_lines_properly() {
            logger().reinitialize();
            logger().log_without_date("first\n\n\nsecond");
            logger().log_without_date("third");
            logger().log_without_date("fourth");

            let output = strip_color_from_string(logger().get_history_buffer_as_string());
            assert_eq!(output, "first\n\n\nsecond\nthird\nfourth")
        }
    }
}
