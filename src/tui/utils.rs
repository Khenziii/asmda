use comfy_table::{Cell, CellAlignment};
use crossterm::terminal::size;
use crate::utils::terminal::strip_color_from_string;

// Splits strings with newline characters into new lines, and then splits them up again if they're too long.
pub fn format_new_rows(rows: Vec<String>) -> Vec<String> {
    let max_log_length: usize = match size() {
        // The padding looks nice.
        Ok((columns, _)) => (columns - 1) as usize,
        Err(_) => 100,
    };

    let mut splitted_rows = Vec::new();
    for row in rows {
        splitted_rows.extend(row.split('\n').map(|s| s.to_string()));
    }

    let mut formatted_rows = Vec::new();
    for row in splitted_rows {
        let mut current_string = String::new();
        for character in row.chars() {
            current_string.push(character);
            
            // We need to remove the ANSI codes to estimate width correctly.
            let visible_string_length = strip_color_from_string(current_string.clone()).len();

            if visible_string_length >= max_log_length {
                formatted_rows.push(current_string.clone());
                current_string = String::new();
            }
        }

        if !current_string.is_empty() {
            formatted_rows.push(current_string);
        }
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
