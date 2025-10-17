use crate::environment::{environment, types::RunningEnvironment};
use crate::tui;
use colored::Colorize;
use once_cell::sync::OnceCell;
use std::io::{Cursor, Write};
use std::sync::{Mutex, MutexGuard};

pub struct Logger {
    history_buffer: Cursor<Vec<u8>>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            history_buffer: Cursor::new(Vec::new()),
        }
    }
    
    fn write(&mut self, log: String) {
        writeln!(self.history_buffer, "{}", log)
            .expect("Failed to write to history buffer! Logger's history won't be accessible.");
        
        let mut interface = tui::tui();
        interface.add_row(log);
    }

    pub fn debug(&mut self, log: &str) {
        let config = environment();
        if config.metadata.running_environment == RunningEnvironment::Production {
            return;
        }

        let str = format!("{}", &log.blue());
        self.write(str);
    }

    pub fn log(&mut self, log: &str) {
        let str = format!("{}", &log.white());
        self.write(str);
    }

    pub fn warn(&mut self, log: &str) {
        let str = format!("{}", &log.yellow());
        self.write(str);
    }

    pub fn error(&mut self, log: &str) {
        let str = format!("{}", &log.red());
        self.write(str);
    }

    pub fn get_history_buffer_as_string(&self) -> String {
        String::from_utf8(self.history_buffer.clone().into_inner())
            .unwrap()
            .trim_end()
            .to_string()
    }
}

static LOGGER: OnceCell<Mutex<Logger>> = OnceCell::new();

fn get_logger() -> &'static Mutex<Logger> {
    LOGGER.get_or_init(|| Mutex::new(Logger::new()))
}

pub fn logger() -> MutexGuard<'static, Logger> {
    get_logger().lock().unwrap()
}
