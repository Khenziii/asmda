use crate::{environment, init_command_option};
use super::CommandOption;

async fn callback() {
    let config = environment::environment();
    println!("ASMDA v{}", config.metadata.program_version)
}

init_command_option!(
    vec!["-v", "--version"],
    "Displays program's current version.",
    callback
);
