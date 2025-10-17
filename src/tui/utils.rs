use crossterm::terminal::size;

// TODO: wrap too long logs, instead of trimming them.

// Splits strings with newline characters into new lines, and trims them if they're too long.
pub fn format_new_rows(rows: Vec<String>) -> Vec<String> {
    let mut formatted_rows = Vec::new();
    let max_log_length: usize = match size() {
        // We're multiplying this 2 times as crossterm seems to underestimate how large terminals
        // really are ;-;. I'll implement some text wrapping later and ditch this anyway.
        Ok((_, rows)) => (rows as f32 * 2.0) as usize,
        Err(_) => 100,
    };

    for row in rows {
        let mut formatted_strings: Vec<String> = row
            .lines()
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
