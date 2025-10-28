use comfy_table::{Cell, CellAlignment};
use crossterm::terminal::size;

// TODO: wrap too long logs, instead of trimming them.

// Splits strings with newline characters into new lines, and trims them if they're too long.
pub fn format_new_rows(rows: Vec<String>) -> Vec<String> {
    let max_log_length: usize = match size() {
        // We're multiplying this 2 times as crossterm seems to underestimate how large terminals
        // really are ;-;. I'll implement some text wrapping later and ditch this anyway.
        Ok((_, rows)) => (rows as f32 * 1.5) as usize,
        Err(_) => 100,
    };

    let mut splitted_rows = Vec::new();
    for row in rows {
        splitted_rows.extend(row.split('\n').map(|s| s.to_string()));
    }

    let mut formatted_rows = Vec::new();
    for row in splitted_rows {
        let formatted_row: String;

        let taken_chars: String = row.chars().take(max_log_length).collect();
        if taken_chars.len() == max_log_length {
            formatted_row = format!("{}...", taken_chars);
        } else {
            formatted_row = taken_chars;
        }

        formatted_rows.push(formatted_row);
    }

    formatted_rows
}

pub fn get_centered_cell_from_string(string: &str) -> Cell {
    Cell::new(string).set_alignment(CellAlignment::Center)
}

#[cfg(test)]
mod tests {
    mod tui_utils {
        use super::super::*;

        #[test]
        fn new_rows_format() {
            let new_row = String::from(
                "Environment {
    metadata: Metadata {
        running_environment: Development,
        database_path: 'asmda.sqlite',
    },
},",
            );
            let new_rows = vec![new_row];

            let formatted_new_rows = format_new_rows(new_rows);
            assert_eq!(
                formatted_new_rows,
                [
                    "Environment {",
                    "    metadata: Metadata {",
                    "        running_environment: Development,",
                    "        database_path: 'asmda.sqlite',",
                    "    },",
                    "},",
                ]
            );
        }
    }
}
