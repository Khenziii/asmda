use super::CommandOption;
use crate::utils::types::AsyncOutput;

static HELP_MESSAGE_HEADER: &str = "
Usage:
$ asmda [option]
";

static HELP_MESSAGE_FOOTER: &str = "
ASMDA by Khenzii <khenzii@khenzii.dev>
https://asmda.khenzii.dev
";

fn format_option_to_string(string_identifiers: &Vec<String>, description: &String) -> String {
    format!("{} - {}", string_identifiers.join("/"), description)
}

// The `init_command_option!` macro is not used here, contrary to other options, as we're adding
// custom logic to values used to construct the `CommandOption` object.
pub fn get_option(context: &Vec<CommandOption>) -> CommandOption {
    let self_string_identifiers = vec![String::from("-h"), String::from("--help")];
    let self_description = String::from("Shows this message.");

    let mut options_string: Vec<String> = context
        .iter()
        .map(|option| format_option_to_string(&option.string_identifiers, &option.description))
        .collect();
    options_string.push(format_option_to_string(&self_string_identifiers, &self_description));
    let help_message = format!("{}\n{}\n{}", HELP_MESSAGE_HEADER.trim_start(), options_string.join("\n"), HELP_MESSAGE_FOOTER.trim_end());
    
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
