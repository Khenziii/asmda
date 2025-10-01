use crate::environment::{environment, types::RunningEnvironment};
use colored::Colorize;

pub fn debug(log: &str) {
    let config = environment();
    if config.running_environment == RunningEnvironment::Production {
        return;
    }

    println!("{}", &log.blue());
}

pub fn log(log: &str) {
    println!("{}", &log);
}

pub fn warn(log: &str) {
    println!("{}", &log.yellow());
}

pub fn error(log: &str) {
    println!("{}", &log.red());
}
