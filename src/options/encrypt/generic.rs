use crate::environment::constants::EnvironmentVariable;
use std::io::{self, Write};
use strum::IntoEnumIterator;

// This should be used before printing any armored values returned by this option.
pub fn format_armored_value(value: String) -> String {
    value.clone().replace("\n", "\\n")
}

pub fn ask_for_string_without_prompt() -> String {
    io::stdout().flush().unwrap();
    
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line!");

    value
}

pub fn ask_for_string() -> String {
    print!("> ");
    ask_for_string_without_prompt()
}

pub fn ask_for_multiline_armored_string() -> String {
    let mut result: Vec<String> = vec![];
    let mut index = 0;

    loop {
        let line;
        if index == 0 {
            line = ask_for_string();
        } else {
            line = ask_for_string_without_prompt();
        }

        let previous_line_was_armor;
        if index == 0 {
            previous_line_was_armor = false;
        } else {
            previous_line_was_armor = result[index - 1].starts_with("-----");
        }
        
        if line.trim_end().is_empty() && !previous_line_was_armor {
            break;
        }

        result.push(line);
        index += 1;
    }

    result.join("").trim_end().to_string()
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
