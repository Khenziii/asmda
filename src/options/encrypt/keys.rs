use crate::options::encrypt::generic::{
    ask_for_multiline_armored_string, ask_for_string, format_armored_value,
};
use crate::utils::exit::exit;
use pgp::native::{SignedSecretKey, types::SecretKeyTrait};
use pgp::{gen_key_pair, read_skey_from_string};
use std::fs;

pub fn get_key_string_by_path() -> String {
    println!("Input the path:");
    let mut path = ask_for_string();
    path = path.trim().to_string();

    fs::read_to_string(path).unwrap_or_else(|_| {
        println!("Passed invalid path! Couldn't read it. Please try again.");
        get_key_string_by_path()
    })
}

pub fn check_if_passphrase_matches_key(key: &mut SignedSecretKey, passphrase: String) -> bool {
    key.unlock(|| passphrase, |_| Ok(())).is_ok()
}

pub fn ask_for_key_passphrase(key: SignedSecretKey) -> String {
    println!("Enter the passphrase of currently used private key:");
    let passphrase = ask_for_string();

    if check_if_passphrase_matches_key(&mut key.clone(), passphrase.clone().trim().to_string()) {
        passphrase
    } else {
        println!("Invalid passphrase passed! Please try again.");
        ask_for_key_passphrase(key)
    }
}

pub async fn generate_key_pair() -> SignedSecretKey {
    loop {
        println!("Enter the passphrase of private key to generate:");
        let passphrase_string = ask_for_string();
        let passphrase = passphrase_string.trim();

        if passphrase.is_empty() {
            println!("Key's passphrase can't be empty! Try again.");
            continue;
        }

        println!(
            "Enter the email of private key to generate (leave empty for `email@domain.com`, this doesn't matter much in our use case):"
        );
        let email_string = ask_for_string();
        let mut email = email_string.trim();

        if email.is_empty() {
            email = "email@domain.com";
        }

        println!(
            "generating a key pair with email: {} and passphrase: {}",
            email, passphrase
        );
        let key_pair = gen_key_pair(email, passphrase).await;
        if let Err(error) = key_pair {
            println!("Failed to generate a key pair! Please try again.");
            println!("Error: {}", error);
            continue;
        }

        let (key, _) = key_pair.unwrap();
        return key;
    }
}

pub async fn ask_for_key() -> SignedSecretKey {
    loop {
        println!(
            "No key is currently defined via `SECRETS_DECRYPTION_KEY` environment variable! You can either:"
        );
        println!("1. Generate a new one");
        println!("2. Pass the path to already present one (you'll need to export it with armor)");
        println!("3. Exit and define the `SECRETS_DECRYPTION_KEY` variable manually. *");
        println!("4. Use a helper utility for manually defining encrypted secrets.");
        println!("* For more guidance on how to do this, please use option 4. ");
        let choice = ask_for_string();

        match choice.trim() {
            "1" => return generate_key_pair().await,
            "2" => {
                let key_string = get_key_string_by_path();
                match read_skey_from_string(key_string).await {
                    Ok(v) => return v,
                    Err(_) => {
                        println!(
                            "Passed invalid path! Couldn't construct key from file's content. Please try again."
                        );
                        continue;
                    }
                }
            }
            "3" => {
                exit();
                unreachable!();
            }
            "4" => {
                println!(
                    "If you'll encounter any issues with the first 2 options, you can always define the secrets manually. Here are the steps in order which will help you to do so:"
                );
                println!();
                println!("```shell");
                println!(
                    "$ gpg --full-generate-key # Generates a new key pair. Make sure to choose the \"RSA and RSA\" option."
                );
                println!(
                    "$ gpg --list-secret-keys --keyid-format LONG # Spot your key, and grab the long ID near the \"sec\" section."
                );
                println!(
                    "$ gpg -a --export-secret-keys [KEY_ID] > key.asc # Export your key and copy its content."
                );
                println!("```");
                println!();
                println!(
                    "Now that you have your key created and exported, you should let ASMDA know about it. To do this, define `SECRETS_DECRYPTION_KEY` and `SECRETS_DECRYPTION_KEY_PASSPHRASE` environment variables. Using those values, the program decrypts every other encrypted variable."
                );
                println!(
                    "IMPORTANT: Before you do that however, please note that all environment variables must span only a single line. This is why you should replace all newline characters with `\\n` (ASMDA is expecting this and will reconstruct the actual values). In order to do this, you can paste the values spanning multiple lines (such as `SECRETS_DECRYPTION_KEY` and all other encrypted secrets) below. Please note, that the string should be unformatted (*don't sanitize it if your terminal asks you*) and that you'll need to click enter a couple of times."
                );

                let value = ask_for_multiline_armored_string();
                println!("{}", value);
                let formatted_value = format_armored_value(value);

                println!("{}", formatted_value);
                println!();
                println!(
                    "Done! You can now use your value for defining armored environment variables."
                );

                exit();
                unreachable!();
            }
            _ => {
                println!("Made invalid choice! Please try again.");
                continue;
            }
        }
    }
}
