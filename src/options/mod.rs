pub mod run;
pub mod logs;
pub mod help;
pub mod version;
pub mod macros;

use crate::utils::types::AsyncFn;
use std::env;

pub struct CommandOption {
    string_identifiers: Vec<String>,
    callback: AsyncFn,
    description: String,
}

pub struct OptionsHandler {
    options: Vec<CommandOption>,
}

impl Default for OptionsHandler {
    fn default() -> Self {
        OptionsHandler::new()
    }
}

impl OptionsHandler {
    pub fn new() -> Self {
        let mut options: Vec<CommandOption> = vec![run::get_option(), logs::get_option(), version::get_option()];
        let help_option = help::get_option(&options);
        options.push(help_option);

        OptionsHandler { options }
    }

    pub async fn handle(&self) {
        let args: Vec<String> = env::args().collect();

        // No options passed.
        if args.len() == 1 {
            let run_callback = run::get_option().callback;
            return run_callback().await;
        }

        let passed_string = args[1].clone();
        for supported_option in &self.options {
            if !supported_option.string_identifiers.iter().any(|v| *v == passed_string) { continue };

            let handler = &supported_option.callback;
            handler().await;
            return;
        }

        panic!("Unknown option passed! Use `-h` to view all supported ones.")
    }
}
