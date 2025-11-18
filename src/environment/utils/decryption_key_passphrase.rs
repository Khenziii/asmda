use crate::environment::utils::generic::as_boolean;
use crate::environment::utils::environment::get_env_var_by_with_potential_fallback;
use crate::environment::constants::EnvironmentVariable;
use once_cell::sync::OnceCell;
use rpassword::read_password;
use std::io::{self, Write};

static DECRYPTION_KEY_PASSPHRASE: OnceCell<Option<String>> = OnceCell::new();

pub fn decryption_key_passphrase() -> &'static Option<String> {
    DECRYPTION_KEY_PASSPHRASE.get_or_init(|| {
        let using_encryption_str =
            get_env_var_by_with_potential_fallback(EnvironmentVariable::SecretsAreEncrypted);
        let using_encryption = as_boolean(using_encryption_str);
        if !using_encryption {
            return None;
        }

        let option_key_passphrase = get_env_var_by_with_potential_fallback(
            EnvironmentVariable::SecretsDecryptionKeyPassphrase,
        );
        let key_passphrase = match option_key_passphrase {
            Some(v) => v,
            None => {
                println!("You're using the `SECRETS_ARE_ENCRYPTED` option, but decryption key's passphrase has not yet been defined. Please input it below: ");
                print!("> ");
                io::stdout().flush().unwrap();

                read_password().expect("Failed to read the password!")
            }
        };
        Some(key_passphrase)
    })
}
