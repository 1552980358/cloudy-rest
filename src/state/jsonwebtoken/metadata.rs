use jsonwebtoken::Algorithm;
use crate::state::Config;

pub struct Metadata {
    pub algorithm: Option<Algorithm>,
    pub key: Key,
}

pub enum Key {
    Secret(String),
    RsaPEM(String, String),
    RsaDER(String, String),
}

impl Metadata {
    pub fn from_config(config: &Config) -> Self {
        let algorithm = config.algorithm();
        let key = config.key();
        Self { algorithm, key }
    }
}

mod key {
    use crate::str_vec;

    pub fn sign_algorithm() -> Vec<String> {
        str_vec!["jwt", "sign", "alg"]
    }

    pub fn key_secret() -> Vec<String> {
        str_vec!["jwt", "key", "secret"]
    }

    pub fn key_rsa_pem_pri() -> Vec<String> {
        str_vec!["jwt", "key", "rsa-pem", "pri"]
    }

    pub fn key_rsa_pem_pub() -> Vec<String> {
        str_vec!["jwt", "key", "rsa-pem", "pub"]
    }

    pub fn key_rsa_der_pri() -> Vec<String> {
        str_vec!["jwt", "key", "rsa-der"]
    }
    
    pub fn key_rsa_der_pub() -> Vec<String> {
        str_vec!["jwt", "key", "rsa-der"]
    }

}

trait MetadataConfig {
    fn algorithm(&self) -> Option<Algorithm>;
    fn key(&self) -> Key;
}

impl MetadataConfig for Config {

    fn algorithm(&self) -> Option<Algorithm> {
        self.get(key::sign_algorithm())
            .map(|algorithm| {
                match algorithm.as_str() {
                    "HS256" => Algorithm::HS256,
                    "HS384" => Algorithm::HS384,
                    "HS512" => Algorithm::HS512,
                    "RS256" => Algorithm::RS256,
                    "RS384" => Algorithm::RS384,
                    "RS512" => Algorithm::RS512,
                    "ES256" => Algorithm::ES256,
                    "ES384" => Algorithm::ES384,
                    "PS256" => Algorithm::PS256,
                    "PS384" => Algorithm::PS384,
                    "PS512" => Algorithm::PS512,
                    _ => panic!(r#"Panic: Unknown JWT algorithm "{algorithm}"."#),
                }
            })
    }

    fn key(&self) -> Key {
        if let Some(private_key) = self.get(key::key_rsa_pem_pri()) {
            if let Some(public_key) = self.get(key::key_rsa_pem_pub()) {
                return Key::RsaPEM(public_key.to_string(), private_key.to_string())
            }
        }

        if let Some(private_key) = self.get(key::key_rsa_der_pri()) {
            if let Some(public_key) = self.get(key::key_rsa_der_pub()) {
                return Key::RsaDER(public_key.to_string(), private_key.to_string())
            }
        }

        let Some(secret) = self.get(key::key_secret()) else {
            panic!("Panic: JWT signing/verifying key(s) is missing.");
        };
        Key::Secret(secret.clone())
    }

}