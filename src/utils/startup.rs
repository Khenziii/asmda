use crate::environment;
use crate::logger;
use rustls;

pub fn show_environment_if_in_dev_env() {
    let config = environment::environment();
    let config_stringified = format!("{:#?}", config);

    logger::debug("Current environment:");
    logger::debug(&config_stringified);
}

pub fn install_crypto_ring_default_provider() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider!");
}

pub fn startup() {
    install_crypto_ring_default_provider();
    show_environment_if_in_dev_env();
}

#[cfg(test)]
mod tests {
    mod startup {
        use super::super::*;

        // This could fail as `install_default` is executed during runtime, so the compiler isn't
        // able to catch all the issues possibly arising from it. If for example some of our
        // dependencies were misconfigured, this test would come in handy.
        #[test]
        fn runs() {
            startup();
        }
    }
}
