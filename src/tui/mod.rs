pub mod table;
pub mod types;
pub mod utils;

use crate::utils::logs::{set_logs_to_string_array, validate_log_directory_setup};
use crate::utils::terminal::{clear_previous_lines, println, strip_color_from_strings};
use crossterm::terminal::size;
use once_cell::sync::OnceCell;
use std::sync::{Mutex, MutexGuard};
use types::NewRowCallback;
use utils::format_new_rows;

fn remove_last_entry_from_vector<T>(vector: &mut Vec<T>) {
    let current_amount_of_entries = vector.len();
    if current_amount_of_entries > 0 {
        vector.truncate(current_amount_of_entries - 1);
    } else {
        vector.clear();
    }
}

fn remove_last_entries_from_vector<T>(vector: &mut Vec<T>, amount: usize) {
    for _ in 0..amount {
        remove_last_entry_from_vector(vector);
    }
}

#[derive(Clone)]
pub struct TerminalUserInterface {
    rows: Vec<String>,
    new_rows_callbacks: Vec<NewRowCallback>,
    // Whether the table should keep updating. This is turned off for example when the program is
    // suspended, and we don't want anything writing to stanard output.
    is_active: bool,
    sync_to_log_file_on_update: bool,
    // If positive, user is viewing logs while being further to the bottom than the last line.
    // If negative, the user has scrolled top. This value represents the amount of lines that
    // have been scrolled.
    current_cursor_offset: i64,
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
            current_cursor_offset: 0,
            sync_to_log_file_on_update,
        }
    }

    fn print(&self) {
        let mut rows = self.rows.clone();

        if self.current_cursor_offset > 0 {
            for _ in 0..self.current_cursor_offset {
                rows.push(String::from(""));
            }
        } else {
            remove_last_entries_from_vector(&mut rows, (-self.current_cursor_offset).max(0) as usize);
        }

        for row in &rows {
            println(row);
        }
    }

    pub fn set_is_active(&mut self, new_is_active: bool) {
        self.is_active = new_is_active;
    }

    // If the total height of the TUI has changed, you'll need to pass `previous_height` in order
    // to keep everything synchronised. If it's the same, passing just `None` is completely fine.
    pub fn rerender(&self, previous_height: Option<usize>) {
        if self.sync_to_log_file_on_update {
            // Write current TUI state to log file.
            set_logs_to_string_array(strip_color_from_strings(self.rows.clone()));
        }

        if !self.is_active {
            return;
        }

        let current_height = self.calculate_height_including_scroll(self.get_height(), self.current_cursor_offset);
        if let Some(previous_height_raw) = previous_height {
            let height_difference = current_height.checked_sub(previous_height_raw).unwrap_or(0);

            for _ in 0..height_difference {
                println("");
            }
        }

        clear_previous_lines(current_height, None);
        self.print();
    }

    pub fn get_height(&self) -> usize {
        self.rows.len()
    }

    fn calculate_height_including_scroll(&self, tui_height: usize, cursor_offset: i64) -> usize {
        (tui_height as isize + cursor_offset as isize).max(0) as usize
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
            let height_including_scroll = self.calculate_height_including_scroll(current_tui_height, self.current_cursor_offset);
            self.rerender(Some(height_including_scroll));
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
        remove_last_entry_from_vector(&mut self.rows);
    }

    pub fn remove_last_rows(&mut self, amount: usize) {
        remove_last_entries_from_vector(&mut self.rows, amount);
    }

    pub fn get_rows(&self) -> Vec<String> {
        self.rows.clone()
    }

    pub fn reinitialize(&mut self) {
        *self = TerminalUserInterface::new(self.sync_to_log_file_on_update);
    }

    pub fn get_current_cursor_offset(&self) -> i64 {
        self.current_cursor_offset
    }

    pub fn set_current_cursor_offset(&mut self, new_cursor_offset: i64) {
        let rows_in_terminal = match size() {
            Ok((_, rows)) => rows - 1,
            Err(_) => 50,
        };
        // There's nothing to scroll to.
        if self.get_height() <= rows_in_terminal as usize { return };
        // Tried to scroll too far down.
        if new_cursor_offset > 0 { return };
        // Tried to scroll too far up.
        if (-new_cursor_offset) as usize > self.get_height() - rows_in_terminal as usize { return };

        let old_cursor_offset = self.current_cursor_offset;
        self.current_cursor_offset = new_cursor_offset;
        let height_including_scroll = self.calculate_height_including_scroll(self.get_height(), old_cursor_offset);
        self.rerender(Some(height_including_scroll));
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
