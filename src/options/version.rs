use crate::environment;

use super::CommandOption;

async fn callback() {
    let config = environment::environment();
    println!("ASMDA v{}", config.metadata.program_version)
}

pub fn get_option() -> CommandOption {
    CommandOption {
        string_identifiers: vec![String::from("-v"), String::from("--version")],
        callback: Box::new(|| Box::pin(callback())),
        description: String::from("Displays program's current version."),
    }
}
