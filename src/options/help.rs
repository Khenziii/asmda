use super::CommandOption;
use crate::utils::types::AsyncOutput;
use crate::environment::environment;

static HELP_MESSAGE_HEADER: &str = "
Usage:
$ asmda [option]
";

fn get_help_message_footer() -> String {
    format!("\n\nASMDA v{} by Khenzii <khenzii@khenzii.dev>", environment().metadata.program_version)
}

static HELP_MESSAGE_END: &str = "
https://asmda.khenzii.dev
";

fn format_option_to_string(string_identifiers: &[String], description: &String) -> String {
    format!("{} - {}", string_identifiers.join("/"), description)
}

// The `init_command_option!` macro is not used here, contrary to other options, as we're adding
// custom logic to values used to construct the `CommandOption` object.
pub fn get_option(context: &[CommandOption]) -> CommandOption {
    let self_string_identifiers = vec![String::from("-h"), String::from("--help")];
    let self_description = String::from("Shows this message.");

    let mut options_string: Vec<String> = context
        .iter()
        .map(|option| format_option_to_string(&option.string_identifiers, &option.description))
        .collect();
    options_string.push(format_option_to_string(&self_string_identifiers, &self_description));
    let options_help_part = format!("\n{}", options_string.join("\n"));
    let help_message = format!("{}{}{}{}", HELP_MESSAGE_HEADER.trim_start(), options_help_part, get_help_message_footer(), HELP_MESSAGE_END.trim_end());
    
    let callback = Box::new(move || {
        let value = help_message.clone();

        Box::pin(async move {
            println!("{}", value);
        }) as AsyncOutput
    });

    CommandOption {
        string_identifiers: self_string_identifiers,
        description: self_description,
        callback,
    }
}
