use super::CommandOption;

async fn callback() {
    println!("Showing the logs!");
}

pub fn get_option() -> CommandOption {
    CommandOption {
        string_identifiers: vec![String::from("logs")],
        callback: Box::new(|| Box::pin(callback())),
    }
}
