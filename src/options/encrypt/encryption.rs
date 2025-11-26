use crate::utils::encryption::EncryptionManager;
use crate::options::encrypt::generic::ask_for_string;
use pgp::native::SignedSecretKey;

pub fn ask_for_value_to_encrypt() -> String {
    println!("Enter the value which you'd like to encrypt.");
    let value = ask_for_string();

    if value.is_empty() {
        println!("Encrypting empty strings is not possible. Please try again.");
        return ask_for_value_to_encrypt();
    }

    value
}

pub async fn get_encrypted_value(key: SignedSecretKey, passphrase: String) -> String {
    let value_to_encrypt = ask_for_value_to_encrypt();
    let encryption_manager = EncryptionManager::new(key.to_armored_string(None).unwrap(), passphrase).await;

    encryption_manager.encrypt(value_to_encrypt).await
}
