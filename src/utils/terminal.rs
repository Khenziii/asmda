use crossterm::{ExecutableCommand, cursor, terminal};
use std::io::{Stdout, stdout};

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
