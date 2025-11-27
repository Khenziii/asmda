use pgp::{
    decrypt as pgp_decrypt, encrypt as pgp_encrypt,
    native::{SignedPublicKey, SignedSecretKey, types::SecretKeyTrait},
    read_skey_from_string,
};

pub struct EncryptionManager {
    key: SignedSecretKey,
    public_key: SignedPublicKey,
    key_password: String,
}

impl EncryptionManager {
    pub async fn new(key_str: String, key_password_arg: String) -> Self {
        let key = read_skey_from_string(key_str)
            .await
            .expect("Failed to create key from String!");
        let key_password = key_password_arg.clone().trim().to_string();
        let public_key = key
            .public_key()
            .sign(&key, || key_password.clone())
            .expect("Failed to generate public key based on the secret one!");
        Self {
            key,
            public_key,
            key_password,
        }
    }

    pub async fn decrypt(&self, encrypted: String) -> String {
        let decrypted_bytes = pgp_decrypt(self.key.clone(), &self.key_password, encrypted.into())
            .await
            .expect("Failed to decrypt passed string!");
        String::from_utf8(decrypted_bytes)
            .unwrap()
            .trim_end()
            .to_string()
    }

    pub async fn encrypt(&self, raw: String) -> String {
        let encrypted_bytes = pgp_encrypt(vec![self.public_key.clone()], raw.into_bytes())
            .await
            .expect("Failed to encrypt passed string!");
        String::from_utf8(encrypted_bytes)
            .unwrap()
            .trim_end()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    mod decrypt {
        use super::super::*;

        static TEST_PRIVATE_KEY: &str = "
-----BEGIN PGP PRIVATE KEY BLOCK-----

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
=tZdn
-----END PGP PRIVATE KEY BLOCK-----
";
        static TEST_PRIVATE_KEY_PASSWORD: &str = "test_private_key_password";
        static TEST_ENCRYPTED_MESSAGE: &str = "
-----BEGIN PGP MESSAGE-----

hQGMA2hxj2cLZDw1AQwApJBGwIWkF39EyBbNcVFkgeWsm2LSd0ZnvMHlzdDuGfSk
wud/u1dUhmPFT/v+m+/ffxcIhFrAUWeqZ9nYDJZ3o79cQV7iSf4BhvnExrY/BJ2O
IFCqcccSA2W0qkFcB+h+BLXE/RjnQhLbTRRt+fD8Yh9mSRCypjThAvyyDnbtXJ87
jsZvXCJy7nJOKb/kAZFl48kH7WZuuLAO6fO+HA7GPRZDcMNkz+8BKDgF7DC1QIsP
YdYU+sSWxtvUjgle5Lvu5Bgq1uxJ4/jJ+gz5TxhNQRG8qObxgD53A+YF0pVfvI9k
Kc4VAVUAFGZdk4jaUoBlHioV3m3nLXEuwcZXqMFr+3B11arp4I1PGcKLd8whyjvr
p/i2S3p0BytSXTcVQVf9nPlYLdV9VkeweEI/i1/PxZSBZ91j0NvxSHip238/4RhL
/tCBrGFvA+bV3ZlrLS4deXKn57hPuwuvX+rseKT7WIUl2cdOyg3QjoBToqEpvclg
zN97LZpXsLzANKqXEcWA0kwBJNlrBe68fDPvq23AIRRTdK+USzItQb0b+gX5YhfJ
xvqCU/aKn1UoOkqcfJ820sbW+/2MPMYhC9WcaTiwdX9efVJWqlUXyfSqXJPQ
=caX1
-----END PGP MESSAGE-----
";
        static TEST_RAW_MESSAGE: &str = "Komm, susser Tod";

        #[tokio::test]
        async fn decrypts_correctly() {
            let test_private_key = TEST_PRIVATE_KEY.to_string();
            let test_private_key_password = TEST_PRIVATE_KEY_PASSWORD.to_string();
            let test_encrypted_message = TEST_ENCRYPTED_MESSAGE.to_string();
            let test_raw_message = TEST_RAW_MESSAGE.to_string();

            let encryption_manager =
                EncryptionManager::new(test_private_key, test_private_key_password).await;
            let decrypted_raw_message = encryption_manager.decrypt(test_encrypted_message).await;

            assert_eq!(test_raw_message, decrypted_raw_message);
        }

        #[tokio::test]
        async fn encrypts_correctly() {
            let test_private_key = TEST_PRIVATE_KEY.to_string();
            let test_private_key_password = TEST_PRIVATE_KEY_PASSWORD.to_string();
            let test_raw_message = TEST_RAW_MESSAGE.to_string();

            let encryption_manager =
                EncryptionManager::new(test_private_key, test_private_key_password).await;
            let encrypted_message = encryption_manager.encrypt(test_raw_message.clone()).await;
            let decrypted_raw_message = encryption_manager.decrypt(encrypted_message).await;

            assert_eq!(test_raw_message, decrypted_raw_message);
        }
    }
}
