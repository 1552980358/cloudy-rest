use chrono::{DateTime, Utc, Duration};
use jsonwebtoken::{
    errors::Error,
    DecodingKey,
    EncodingKey,
    Header,
    Validation
};
use serde::{Deserialize, Serialize};

use crate::state::Config;

mod metadata;
use metadata::{Metadata, Key};

pub struct JsonWebToken {
    header: Header,
    validation: Validation,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    duration: Duration,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    #[serde(rename = "jti")]
    pub id: String,
    #[serde(rename = "sub")]
    pub account: String,
    #[serde(rename = "pub")]
    pub public_key: String,
    #[serde(rename = "iat")]
    pub issue: i64,
    #[serde(rename = "exp")]
    pub expiry: i64,
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

        let duration = Duration::milliseconds(metadata.duration);

        Self {
            header,
            validation,
            encoding_key,
            decoding_key,
            duration,
        }
    }

    pub fn expiry_from(&self, timestamp: &DateTime<Utc>) -> DateTime<Utc> {
        *timestamp + self.duration
    }

    pub fn new_claims(
        &self,
        token_id: &String,
        account_id: &String,
        public_key_id: &String,
        issue_timestamp: &DateTime<Utc>
    ) -> Claims {
        Claims {
            id: token_id.clone(),
            account: account_id.clone(),
            public_key: public_key_id.clone(),
            issue: issue_timestamp.timestamp(),
            expiry: self.expiry_from(issue_timestamp).timestamp(),
        }
    }

    pub fn encode_jwt(&self, claims: &Claims) -> Result<String, Error> {
        jsonwebtoken::encode(&self.header, claims, &self.encoding_key)
    }

    pub fn decode_jwt(&self, jwt_str: &String) -> Result<Claims, Error> {
        jsonwebtoken::decode::<Claims>(&*jwt_str, &self.decoding_key, &self.validation)
            // Hide the header, expose the claims only
            .map(|token_data| token_data.claims)
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