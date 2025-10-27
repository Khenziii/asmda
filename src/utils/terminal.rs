use crate::tui;
use crate::tui::TerminalUserInterface;
use crate::tui::table::{Table, tasks_table::item::TasksTableItem, tasks_table::table::TasksTable};
use crossterm::{ExecutableCommand, cursor, terminal};
use std::io::{Stdout, stdout};
use std::sync::Arc;
use strip_ansi_escapes;

// Removes ANSI codes added by `colored` crate used in our logger.
pub fn strip_color_from_string(string: String) -> String {
    let stripped = strip_ansi_escapes::strip(string);
    let stripped_string = String::from_utf8(stripped).unwrap();
    stripped_string
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
        for row in rows {
            println!("{}", row);
        }
    }
}

pub fn setup_tui() {
    let mut table = TasksTable::new();
    let mut tui = tui::tui();

    // TODO: add actual table rows here...
    table.add_item(
        "Test".to_string(),
        TasksTableItem {
            name: "Task name".to_string(),
            next_run: "Next run".to_string(),
        },
    );

    add_table_to_tui(table.clone(), &mut tui, true);

    // Reorders rows after adding them.
    // Assume that we have a TUI with logs at the top and a table at the end (one which should stay
    // there permanently). After we write another row, we'll have: rows --> table --> row. This
    // callback reorders the new row to be placed between old rows and the table, so:
    // rows --> row --> table.
    let new_row_callback = move |tui: &mut TerminalUserInterface, new_rows: Vec<String>| {
        let amount_of_rows_added = new_rows.len();
        let table_height = table.get_height();

        tui.remove_last_rows(amount_of_rows_added);
        tui.remove_last_rows(table_height);

        tui.add_rows(new_rows, false, false);
        add_table_to_tui(table.clone(), tui, false);
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

        #[test]
        fn stripping_colors_from_strings() {
            let log = "hello!".red().to_string();
            let raw_log = strip_color_from_string(log);
            assert_eq!(raw_log, "hello!");

            let logs = vec!["hi!".red().to_string(), "hey!".red().to_string()];
            let raw_logs = strip_color_from_strings(logs);
            assert_eq!(raw_logs, ["hi!", "hey!"]);
        }

        #[test]
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
                    "+------+-----------+----------+",
                    "| ID   | Name      | Next run |",
                    "+=============================+",
                    "| Test | Task name | Next run |",
                    "+------+-----------+----------+",
                ]
            );
        }
    }
}
