use crate::tui;
use crate::tui::TerminalUserInterface;
use crate::tui::table::utils::setup_tasks_table_in_tui;
use crate::tui::table::{Table, tasks_table::table::TasksTable};
use crossterm::{ExecutableCommand, cursor, terminal};
use std::io::{Stdout, stdout};
use std::sync::{Arc, Mutex};
use strip_ansi_escapes;

// We're using raw mode. Terminal won't automatically handle this for us.
pub fn println(string: &str) {
    print!("{}\r\n", string);
}

// Removes ANSI codes added by `colored` crate used in our logger.
pub fn strip_color_from_string(string: String) -> String {
    let stripped = strip_ansi_escapes::strip(string);
    String::from_utf8(stripped).unwrap()
}

pub fn strip_color_from_strings(strings: Vec<String>) -> Vec<String> {
    strings
        .iter()
        .map(|string| strip_color_from_string(string.clone()))
        .collect()
}

pub fn clear_previous_lines(amount: usize, stdout_arg: Option<Stdout>) {
    let mut output_stream = match stdout_arg {
        None => stdout(),
        Some(s) => s,
    };

    for _ in 0..amount {
        output_stream
            .execute(cursor::MoveUp(1))
            .expect("Failed to move to the previous line of standard output!");
        output_stream
            .execute(terminal::Clear(terminal::ClearType::CurrentLine))
            .expect("Failed to clear line of standard output!");
    }
}

pub fn add_table_to_tui<T, U>(table: U, tui: &mut TerminalUserInterface, print: bool)
where
    U: Table<T>,
{
    let rows = table.as_string_array();
    tui.add_rows(rows.clone(), false, false);

    if print {
        tui.rerender(None);
    }
}

pub fn refresh_table_in_tui<T, U>(table: U, tui: &mut TerminalUserInterface)
where
    U: Table<T>,
{
    // We can simply do this, as there's always a single table at the bottom of the TUI that has
    // always the exact same height since the program's start, all the way to the end.
    let previous_height = table.get_height();
    tui.remove_last_rows(previous_height);

    add_table_to_tui(table, tui, true);
}

pub fn setup_tui() {
    let table = Arc::new(Mutex::new(TasksTable::new()));

    setup_tasks_table_in_tui(Arc::clone(&table));
    let mut tui = tui::tui();

    // Reorders rows after adding them.
    // Assume that we have a TUI with logs at the top and a table at the end (one which should stay
    // there permanently). After we write another row, we'll have: rows --> table --> row. This
    // callback reorders the new row to be placed between old rows and the table, so:
    // rows --> row --> table.
    let callback_table_pointer = Arc::clone(&table);
    let new_row_callback = move |tui: &mut TerminalUserInterface, new_rows: Vec<String>| {
        let callback_table = callback_table_pointer.lock().unwrap();

        let amount_of_rows_added = new_rows.len();
        let table_height = callback_table.get_height();

        tui.remove_last_rows(amount_of_rows_added);
        tui.remove_last_rows(table_height);

        tui.add_rows(new_rows, false, false);
        add_table_to_tui(callback_table.clone(), tui, false);
    };
    let wrapped_new_row_callback = Arc::new(new_row_callback);

    tui.add_new_rows_callback(wrapped_new_row_callback);
}

#[cfg(test)]
mod tests {
    mod terminal {
        use super::super::*;
        use crate::logger;
        use crate::tui::tui;
        use colored::Colorize;
        use serial_test::serial;

        #[test]
        #[serial]
        fn stripping_colors_from_strings() {
            let log = "hello!".red().to_string();
            let raw_log = strip_color_from_string(log);
            assert_eq!(raw_log, "hello!");

            let logs = vec!["hi!".red().to_string(), "hey!".red().to_string()];
            let raw_logs = strip_color_from_strings(logs);
            assert_eq!(raw_logs, ["hi!", "hey!"]);
        }

        #[test]
        #[serial]
        fn default_tui_setup_works() {
            logger().reinitialize();
            tui().reinitialize();
            setup_tui();
            logger().log("Starting up...");

            let output = strip_color_from_strings(tui().get_rows());
            assert_eq!(
                output,
                [
                    "Starting up...",
                    "╭────┬────────────┬──────────╮",
                    "│ ID │    Name    │ Next run │",
                    "╞════╪════════════╪══════════╡",
                    "│  1 │ letterboxd │    0s    │",
                    "╰────┴────────────┴──────────╯",
                ]
            );
        }
    }
}
