use crossterm::ExecutableCommand;
use crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};
use std::{io::stdout, process};

pub fn leave_alternate_terminal_screen_mode() {
    let mut output_stream = stdout();
    output_stream
        .execute(LeaveAlternateScreen)
        .expect("Failed to leave alternate screen mode! Terminal might behave weirdly.");
}

pub fn disable_terminal_raw_mode() {
    disable_raw_mode().unwrap();
}

pub fn return_zero() {
    process::exit(0);
}

// This should be *always* called when the program exits.
pub fn exit() {
    leave_alternate_terminal_screen_mode();
    disable_terminal_raw_mode();
    return_zero();
}
