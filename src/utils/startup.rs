use rustls;

pub fn startup() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider!");
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
