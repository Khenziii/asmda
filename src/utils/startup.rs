use crate::environment;
use crate::input::user_input_handler;
use crate::logger::logger;
use crate::signals::SignalsHandler;
use crate::status::status_server;
use crate::utils::tests::is_test_environment;
use crossterm::ExecutableCommand;
use crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use rustls;
use std::io::stdout;

pub fn show_environment_if_in_dev_env() {
    let config = environment::environment();
    let config_stringified = format!("{:#?}", config);

    logger().debug("Current environment:");
    logger().debug(&config_stringified);
}

pub fn install_crypto_ring_default_provider() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider!");
}

pub fn enable_terminal_alternate_screen_mode() {
    let mut output_stream = stdout();
    output_stream
        .execute(EnterAlternateScreen)
        .expect("Failed to enter alternate screen mode! TUI might fail.");
}

pub fn enable_terminal_raw_mode() {
    enable_raw_mode().expect("Failed to enable raw mode! TUI might appear broken.");
}

pub fn setup_user_event_loop() {
    user_input_handler();
}

pub fn setup_signals_event_loop() {
    let handler = SignalsHandler::new();
    handler.run();
}

pub fn setup_status_server() {
    let config = environment::environment();

    if !config.status_server.enable {
        return;
    };

    let server = status_server();
    server.start_non_blocking();
}

pub fn startup() {
    install_crypto_ring_default_provider();
    show_environment_if_in_dev_env();
    if !is_test_environment() {
        enable_terminal_alternate_screen_mode();
        enable_terminal_raw_mode();
        setup_user_event_loop();
        setup_signals_event_loop();
        setup_status_server();
    }
}

#[cfg(test)]
mod tests {
    mod startup {
        use super::super::*;
        use serial_test::serial;

        // This could fail as `install_default` is executed during runtime, so the compiler isn't
        // able to catch all the issues possibly arising from it. If for example some of our
        // dependencies were misconfigured, this test would come in handy.
        #[test]
        #[serial]
        fn runs() {
            startup();
        }
    }
}
