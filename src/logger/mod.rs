// TODO: finish implementing this, add some colors

use crate::environment::{environment, types::RunningEnvironment};

pub fn debug(log: &str) {
    let config = environment();
    if config.running_environment == RunningEnvironment::Production { return }

    println!("{}", log);
}

pub fn log(log: &str) {
    println!("{}", log);
}

pub fn warn() {}
pub fn error() {}
