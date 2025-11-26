use crate::init_command_option;
use crate::environment::{environment, constants::EnvironmentVariable};
use crate::utils::encryption::EncryptionManager;
use crate::utils::exit::exit;
use super::CommandOption;
use pgp::native::types::SecretKeyTrait;
use pgp::{native::SignedSecretKey, read_skey_from_string, gen_key_pair};
use strum::IntoEnumIterator;
use secrecy::ExposeSecret;
use std::io::{self, Write};
use std::fs;

// This should be used before printing any armored values returned by this option.
fn format_armored_value(value: String) -> String {
    value.clone().replace("\n", "\\n")
}

fn ask_for_string() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line!");

    value
}

fn get_key_string_by_path() -> String {
    println!("Input the path:");
    let mut path = ask_for_string();
    path = path.trim().to_string();

    let key_string = fs::read_to_string(path).unwrap_or_else(|_| {
        println!("Passed invalid path! Couldn't read it. Please try again.");
        get_key_string_by_path()
    });
    key_string
}

fn check_if_passphrase_matches_key(key: &mut SignedSecretKey, passphrase: String) -> bool {
    key.unlock(|| passphrase, |_| Ok(())).is_ok()
}

fn ask_for_key_passphrase(key: SignedSecretKey) -> String {
    println!("Enter the passphrase of currently used private key:");
    let passphrase = ask_for_string();

    if check_if_passphrase_matches_key(&mut key.clone(), passphrase.clone()) {
        passphrase
    } else {
        println!("Invalid passphrase passed! Please try again.");
        ask_for_key_passphrase(key)
    }
}

async fn generate_key_pair() -> SignedSecretKey {
    loop {
        println!("Enter the passphrase of private key to generate:");
        let passphrase = ask_for_string();
    
        if passphrase.len() == 0 {
            println!("Key's passphrase can't be empty! Try again.");
            continue;
        }
    
        println!("Enter the email of private key to generate (leave empty for `email@domain.com`, this doesn't matter much in our use case):");
        let mut email = ask_for_string();

        if email == "" {
            email = String::from("email@domain.com");
        }
    
        let key_pair = gen_key_pair(email, passphrase).await;
        if let Err(_) = key_pair {
            println!("Failed to generate a key pair! Please try again.");
            continue;
        }

        let (key, _) = key_pair.unwrap();
        return key;
    }
}

async fn ask_for_key() -> SignedSecretKey {
    loop {
        println!("No key is currently defined via `SECRETS_DECRYPTION_KEY` environment variable! You can either:");
        println!("1. Generate a new one");
        println!("2. Pass the path to already present one");
        println!("3. Exit and define the `SECRETS_DECRYPTION_KEY` variable manually");
        let choice = ask_for_string();
    
        match choice.as_str() {
            "1" => { return generate_key_pair().await },
            "2" => {
                let key_string = get_key_string_by_path();
                match read_skey_from_string(key_string).await {
                    Ok(v) => { return v },
                    Err(_) => {
                        println!("Passed invalid path! Couldn't construct key from file's content. Please try again.");
                        continue;
                    },
                }
            },
            "3" => {
                exit();
                unreachable!();
            },
            _ => {
                println!("Made invalid choice! Please try again.");
                continue;
            }
        }
    }
}

fn ask_for_value_to_encrypt() -> String {
    println!("Enter the value which you'd like to encrypt.");
    let value = ask_for_string();

    if value.len() == 0 {
        println!("Encrypting empty strings is not possible. Please try again.");
        return ask_for_value_to_encrypt();
    }

    value
}

async fn get_encrypted_value(key: SignedSecretKey, passphrase: String) -> String {
    let value_to_encrypt = ask_for_value_to_encrypt();

    let encryption_manager = EncryptionManager::new(key.to_armored_string(None).unwrap(), passphrase).await;
    let encrypted = encryption_manager.encrypt(value_to_encrypt).await;

    encrypted
}

fn ask_for_variable_name() -> String {
    println!("Which variable are you trying to define? (e.g. `LETTERBOXD_PASSWORD`)");
    let variable_name = ask_for_string();

    let all_environment_variables: Vec<EnvironmentVariable> = EnvironmentVariable::iter().collect();
    for environment_variable in all_environment_variables {
        if !(variable_name.trim() == environment_variable.as_str()) {
            continue;
        }

        if !environment_variable.can_be_encrypted() {
            println!("This variable can't be encrypted!");
            return ask_for_variable_name();
        }

        return variable_name;
    }

    println!("Provided variable name doesn't exist! Please try again.");
    ask_for_variable_name()
}

async fn callback() {
    let config = environment();
    let environment_key = config.secrets.decryption_key.clone();
    let environment_key_passphrase = config.secrets.decryption_key_passphrase.clone();

    let mut key;
    if environment_key.is_none() {
        key = ask_for_key().await;
    } else {
        key = match read_skey_from_string(environment_key.unwrap()).await {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid key is set in the environment file. Please delete it (`SECRETS_DECRYPTION_KEY`) and rerun this command.");
                exit();
                unreachable!();
            },
        }
    }

    let key_passphrase: String;
    if environment_key_passphrase.is_none() {
        key_passphrase = ask_for_key_passphrase(key.clone());
    } else {
        key_passphrase = environment_key_passphrase.clone().unwrap().expose_secret().to_string();

        if !check_if_passphrase_matches_key(&mut key, key_passphrase.clone()) {
            println!("Defined passphrase doesn't match currently set key! Please make sure that `SECRETS_DECRYPTION_KEY` can be unlocked by `SECRETS_DECRYPTION_KEY_PASSPHRASE`.");
            exit();
        }
    }

    let variable_name = ask_for_variable_name();
    let encrypted_value = get_encrypted_value(key.clone(), key_passphrase.clone()).await;

    println!("All done! You can insert this into the file containing your environment variables:");
    println!("");
    println!("```");
    println!("SECRETS_ARE_ENCRYPTED=\"true\"");
    println!("{}", format!("SECRETS_DECRYPTION_KEY=\"{}\"", format_armored_value(key.to_armored_string(None).unwrap())));
    println!("{}", format!("SECRETS_DECRYPTION_KEY_PASSPHRASE=\"{}\"", key_passphrase));
    println!("{}", format!("{}=\"{}\"", variable_name.trim().to_string(), format_armored_value(encrypted_value)));
    println!("```");
    println!("");
    println!("Remember to look out for duplicates. You might also want to remove `SECRETS_DECRYPTION_KEY_PASSPHRASE` if you'd like to pass the password via standard input when starting the program. It's oftentimes more secure, as it's not saved on the hard drive.");
}

init_command_option!(
    vec!["encrypt"],
    "Helper utility for generating encrypted secrets that can be used in the environment file. For more info about encrypting secrets please refer to the documentation.",
    callback
);
