use crate::utils::terminal::clear_previous_lines;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex, MutexGuard};
use crossterm::terminal::size;

// TODO: wrap too long logs, instead of trimming them.

// Splits strings with newline characters into new lines, and trims them if they're too long.
fn format_new_rows(rows: Vec<String>) -> Vec<String> {
    let mut formatted_rows = Vec::new();
    let max_log_length: usize = match size() {
        // We're multiplying this 2 times as crossterm seems to underestimate how large terminals
        // really are ;-;. I'll implement some text wrapping later and ditch this anyway.
        Ok((_, rows)) => (rows as f32 * 2.0) as usize,
        Err(_) => 100,
    };

    for row in rows {
        let mut formatted_strings: Vec<String> = row.lines()
            .map(|line| {
                let formatted_line: String = line.chars().take(max_log_length).collect();
                if formatted_line.len() == max_log_length {
                    format!("{}...", formatted_line)
                } else {
                    formatted_line
                }
            })
            .collect();
        formatted_rows.append(&mut formatted_strings);
    }

    formatted_rows
}

type NewRowCallback = Arc<dyn Fn(&mut TerminalUserInterface, Vec<String>) + Send + Sync>;

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
