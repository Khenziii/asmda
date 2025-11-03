use crate::environment::{environment, types::RunningEnvironment};
use crate::utils::time::get_current_formatted_date;
use colored::Colorize;

pub enum LogLevel {
    Debug,
    Log,
    Warn,
    Error,
}

#[derive(Clone)]
pub struct LogBuilder {
    log: String,
    date: Option<String>,
    printable: bool,
}

impl LogBuilder {
    pub fn new(default_content: &str) -> Self {
        Self {
            log: default_content.to_string(),
            date: None,
            printable: true,
        }
    }

    pub fn build(&self) -> String {
        if !self.printable {
            return String::from("");
        }

        if self.date.is_some() {
            return format!("{} > {}", self.date.clone().unwrap(), self.log);
        } else {
            self.log.clone()
        }
    }

    pub fn set_level(&mut self, level: LogLevel) -> Self {
        self.log = match level {
            LogLevel::Debug => self.log.blue().to_string(),
            LogLevel::Log => self.log.white().to_string(),
            LogLevel::Warn => self.log.yellow().to_string(),
            LogLevel::Error => self.log.red().to_string(),
        };
        self.clone()
    }

    pub fn add_date(&mut self) -> Self {
        let current_formatted_date = get_current_formatted_date();
        let colored_current_formatted_date = current_formatted_date.white().to_string();
        self.date = Some(colored_current_formatted_date);
        self.clone()
    }

    pub fn only_in_dev_env(&mut self) -> Self {
        let config = environment();
        if config.metadata.running_environment == RunningEnvironment::Production {
            self.printable = false;
        }
        self.clone()
    }
}
