pub mod log;

use crate::tui;
use crate::utils::startup::create_log_directory_if_missing;
use crate::utils::tests::is_test_environment;
use log::{LogBuilder, LogLevel};
use once_cell::sync::OnceCell;
use std::io::{Cursor, Write};
use std::sync::{Mutex, MutexGuard};

pub struct Logger {
    history_buffer: Cursor<Vec<u8>>,
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger {
    pub fn new() -> Self {
        // We need to call this again here if in test environment, as the usually used `startup`
        // function doesn't run in tests. This prevents any errors caused by that.
        if is_test_environment() {
            create_log_directory_if_missing();
        }

        Logger {
            history_buffer: Cursor::new(Vec::new()),
        }
    }

    pub fn reinitialize(&mut self) {
        *self = Logger::new();
    }

    pub fn get_history_buffer_as_string(&self) -> String {
        String::from_utf8(self.history_buffer.clone().into_inner())
            .unwrap()
            .trim_end()
            .to_string()
    }

    fn add_log_to_history_buffer(&mut self, log: String) {
        writeln!(self.history_buffer, "{}", log)
            .expect("Failed to write to history buffer! Logger's history won't be accessible.");
    }

    fn write(&mut self, log: String) {
        self.add_log_to_history_buffer(log.clone());

        let mut interface = tui::tui();
        interface.add_row(log, true, true);
    }

    pub fn debug(&mut self, log: &str) {
        let log_builder = LogBuilder::new(log)
            .add_date()
            .only_in_dev_env()
            .set_level(LogLevel::Debug);
        self.write(log_builder.build());
    }

    pub fn debug_without_date(&mut self, log: &str) {
        let log_builder = LogBuilder::new(log)
            .only_in_dev_env()
            .set_level(LogLevel::Debug);
        self.write(log_builder.build());
    }

    pub fn log(&mut self, log: &str) {
        let log_builder = LogBuilder::new(log).add_date().set_level(LogLevel::Log);
        self.write(log_builder.build());
    }

    pub fn log_without_date(&mut self, log: &str) {
        let log_builder = LogBuilder::new(log).set_level(LogLevel::Debug);
        self.write(log_builder.build());
    }

    pub fn warn(&mut self, log: &str) {
        let log_builder = LogBuilder::new(log).add_date().set_level(LogLevel::Warn);
        self.write(log_builder.build());
    }

    pub fn warn_without_date(&mut self, log: &str) {
        let log_builder = LogBuilder::new(log).set_level(LogLevel::Warn);
        self.write(log_builder.build());
    }

    pub fn error(&mut self, log: &str) {
        let log_builder = LogBuilder::new(log).add_date().set_level(LogLevel::Error);
        self.write(log_builder.build());
    }

    pub fn error_without_date(&mut self, log: &str) {
        let log_builder = LogBuilder::new(log).set_level(LogLevel::Error);
        self.write(log_builder.build());
    }
}

static LOGGER: OnceCell<Mutex<Logger>> = OnceCell::new();

fn get_logger() -> &'static Mutex<Logger> {
    LOGGER.get_or_init(|| Mutex::new(Logger::new()))
}

pub fn logger() -> MutexGuard<'static, Logger> {
    get_logger().lock().unwrap()
}

#[cfg(test)]
mod tests {
    mod logger {
        use super::super::*;
        use crate::utils::terminal::strip_color_from_string;

        #[test]
        fn reinitialization_works() {
            logger().log_without_date("hi!");
            logger().reinitialize();
            logger().log_without_date("hey!");

            let output = strip_color_from_string(logger().get_history_buffer_as_string());
            assert_eq!(output, "hey!");
        }
    }
}
