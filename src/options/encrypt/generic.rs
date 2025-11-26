use crate::environment::constants::EnvironmentVariable;
use std::io::{self, Write};
use strum::IntoEnumIterator;

// This should be used before printing any armored values returned by this option.
pub fn format_armored_value(value: String) -> String {
    value.clone().replace("\n", "\\n")
}

pub fn ask_for_string() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line!");

    value
}

pub fn ask_for_variable_name() -> String {
    println!("Which variable are you trying to define? (e.g. `LETTERBOXD_PASSWORD`)");
    let variable_name = ask_for_string();

    let all_environment_variables: Vec<EnvironmentVariable> = EnvironmentVariable::iter().collect();
    for environment_variable in all_environment_variables {
        if !(variable_name.trim() == environment_variable.as_str()) {
            continue;
        }

        if !environment_variable.can_be_encrypted() {
            println!("This variable can't be encrypted!");
            return ask_for_variable_name();
        }

        return variable_name;
    }

    println!("Provided variable name doesn't exist! Please try again.");
    ask_for_variable_name()
}
