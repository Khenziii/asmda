pub mod api_wrappers;
pub mod archivers;
pub mod environment;
pub mod input;
pub mod logger;
pub mod options;
pub mod schedule;
pub mod signals;
pub mod status;
pub mod tui;
pub mod utils;

use options::OptionsHandler;

pub async fn run() {
    let options_handler = OptionsHandler::new();
    options_handler.handle().await;
}
