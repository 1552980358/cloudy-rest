use jsonwebtoken::{DecodingKey, EncodingKey, Header};
use jsonwebtoken::{
    DecodingKey,
    EncodingKey,
    Header,
    Validation
};

use crate::state::Config;

mod metadata;
use metadata::{Metadata, Key};

pub struct JsonWebToken {
    header: Header,
    validation: Validation,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    duration: i64,
}

impl JsonWebToken {

    pub fn from_config(config: &Config) -> Self {
        let metadata = Metadata::from_config(config);

        let header = metadata.algorithm
            .map(Header::new)
            .unwrap_or_else(Header::default);
        let validation = metadata.algorithm
            .map(Validation::new)
            .unwrap_or_else(Validation::default);

        let (encoding_key, decoding_key) = match metadata.key {
            Key::Secret(secret) => {
                crypto_keys_processor::of_secret(&secret)
            }
            Key::RsaPEM(public_key, private_key) => {
                crypto_keys_processor::of_rsa_pem(&public_key, &private_key)
            }
            Key::RsaDER(public_key, private_key) => {
                crypto_keys_processor::of_rsa_der(&public_key, &private_key)
            }
        };

        let duration = metadata.duration;

        Self {
            header,
            validation,
            encoding_key,
            decoding_key,
            duration,
        }
    }

}

mod crypto_keys_processor {
    use std::fs::read as read_bytes;
    use jsonwebtoken::{DecodingKey, EncodingKey};

    pub fn of_secret(secret_key: &String) -> (EncodingKey, DecodingKey) {
        let secret_key_bytes = secret_key.as_bytes();
        let encoding_key = EncodingKey::from_secret(&secret_key_bytes);
        let decoding_key = DecodingKey::from_secret(&secret_key_bytes);
        (encoding_key, decoding_key)
    }

    pub fn of_rsa_pem(
        public_key_path: &String,
        private_key_path: &String,
    ) -> (EncodingKey, DecodingKey) {
        let Ok(private_key) = read_bytes(&private_key_path) else {
            panic!(r#"Panic: RSA-PME private key cannot be found from "{private_key_path}"."#);
        };
        let Ok(encoding_key) = EncodingKey::from_rsa_pem(&private_key) else {
            panic!("Panic: Invalid RSA-PEM private key.");
        };

        let Ok(public_key) = read_bytes(&public_key_path) else {
            panic!(r#"Panic: RSA-PME public key cannot be found from "{public_key_path}"."#);
        };
        let Ok(decoding_key) = DecodingKey::from_rsa_pem(&public_key) else {
            panic!("Panic: Invalid RSA-PEM public key.");
        };

        (encoding_key, decoding_key)
    }

    pub fn of_rsa_der(
        public_key_path: &String,
        private_key_path: &String,
    ) -> (EncodingKey, DecodingKey) {
        let Ok(private_key) = read_bytes(&private_key_path) else {
            panic!(r#"Panic: RSA-DER private key cannot be found from "{private_key_path}"."#);
        };
        let encoding_key = EncodingKey::from_rsa_der(&private_key);

        let Ok(public_key) = read_bytes(&public_key_path) else {
            panic!(r#"Panic: RSA-DER private key cannot be found from "{public_key_path}"."#);
        };
        let decoding_key = DecodingKey::from_rsa_der(&public_key);

        (encoding_key, decoding_key)
    }

}