#[macro_export]
macro_rules! init_command_option {
    ($identifiers:expr, $description:expr, $callback:expr) => {
        pub fn get_option() -> CommandOption {
            CommandOption {
                string_identifiers: $identifiers.into_iter().map(|s| String::from(s)).collect(),
                callback: Box::new(|| Box::pin($callback())),
                description: $description.to_string(),
            }
        }
    };
}
