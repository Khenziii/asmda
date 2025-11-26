use crate::options::encrypt::generic::ask_for_string;
use crate::utils::exit::exit;
use pgp::{read_skey_from_string, gen_key_pair};
use pgp::native::{SignedSecretKey, types::SecretKeyTrait};
use std::fs;


pub fn get_key_string_by_path() -> String {
    println!("Input the path:");
    let mut path = ask_for_string();
    path = path.trim().to_string();

    let key_string = fs::read_to_string(path).unwrap_or_else(|_| {
        println!("Passed invalid path! Couldn't read it. Please try again.");
        get_key_string_by_path()
    });
    key_string
}

pub fn check_if_passphrase_matches_key(key: &mut SignedSecretKey, passphrase: String) -> bool {
    key.unlock(|| passphrase, |_| Ok(())).is_ok()
}

pub fn ask_for_key_passphrase(key: SignedSecretKey) -> String {
    println!("Enter the passphrase of currently used private key:");
    let passphrase = ask_for_string();

    if check_if_passphrase_matches_key(&mut key.clone(), passphrase.clone()) {
        passphrase
    } else {
        println!("Invalid passphrase passed! Please try again.");
        ask_for_key_passphrase(key)
    }
}

pub async fn generate_key_pair() -> SignedSecretKey {
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

pub async fn ask_for_key() -> SignedSecretKey {
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
