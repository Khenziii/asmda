mod encryption;
mod generic;
mod keys;

use super::CommandOption;
use crate::environment::environment;
use crate::init_command_option;
use crate::options::encrypt::encryption::get_encrypted_value;
use crate::options::encrypt::generic::{ask_for_variable_name, format_armored_value};
use crate::options::encrypt::keys::{
    ask_for_key, ask_for_key_passphrase, check_if_passphrase_matches_key,
};
use crate::utils::exit::exit;
use pgp::read_skey_from_string;
use secrecy::ExposeSecret;

async fn callback() {
    let config = environment();
    let environment_key = config.secrets.decryption_key.clone();
    let environment_key_passphrase = config.secrets.decryption_key_passphrase.clone();

    let mut key = if let Some(value) = environment_key {
        match read_skey_from_string(value).await {
            Ok(v) => v,
            Err(_) => {
                println!(
                    "Invalid key is set in the environment file. Please delete it (`SECRETS_DECRYPTION_KEY`) and rerun this command."
                );
                exit();
                unreachable!();
            }
        }
    } else {
        ask_for_key().await
    };

    let key_passphrase: String;
    if environment_key_passphrase.is_none() {
        key_passphrase = ask_for_key_passphrase(key.clone());
    } else {
        key_passphrase = environment_key_passphrase
            .clone()
            .unwrap()
            .expose_secret()
            .to_string();

        if !check_if_passphrase_matches_key(&mut key, key_passphrase.clone()) {
            println!(
                "Defined passphrase doesn't match currently set key! Please make sure that `SECRETS_DECRYPTION_KEY` can be unlocked by `SECRETS_DECRYPTION_KEY_PASSPHRASE`."
            );
            exit();
        }
    }

    let variable_name = ask_for_variable_name();
    let encrypted_value = get_encrypted_value(key.clone(), key_passphrase.clone()).await;

    let secrets_decryption_key_line = format!(
        "SECRETS_DECRYPTION_KEY=\"{}\"",
        format_armored_value(key.to_armored_string(None).unwrap())
    );
    let secrets_decryption_key_passphrase_line =
        format!("SECRETS_DECRYPTION_KEY_PASSPHRASE=\"{}\"", key_passphrase);
    let variable_name_and_encrypted_value_line = format!(
        "{}=\"{}\"",
        variable_name.trim(),
        format_armored_value(encrypted_value)
    );
    let generated_message: Vec<&str> = vec![
        "All done! You can insert this into the file containing your environment variables:",
        "",
        "```",
        "SECRETS_ARE_ENCRYPTED=\"true\"",
        &secrets_decryption_key_line,
        &secrets_decryption_key_passphrase_line,
        &variable_name_and_encrypted_value_line,
        "```",
        "",
        "Remember to look out for duplicates. You might also want to remove `SECRETS_DECRYPTION_KEY_PASSPHRASE` if you'd like to pass the password via standard input when starting the program. It's oftentimes more secure, as it's not saved on the hard drive.",
    ];

    for line in generated_message {
        println!("{}", line);
    }
}

init_command_option!(
    vec!["encrypt"],
    "Helper utility for generating encrypted secrets that can be used in the environment file. For more info about encrypting secrets please refer to the documentation.",
    callback
);
