use super::CommandOption;
use crate::environment::environment;
use crate::init_command_option;

async fn callback() {
    println!("ASMDA v{}", environment().metadata.program_version)
}

init_command_option!(
    vec!["-v", "--version"],
    "Displays program's current version.",
    callback
);
