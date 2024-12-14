/**
 * JWT metadata loading from [Config].
 * Detail config keys look at [key].
 **/

use jsonwebtoken::Algorithm;

use crate::state::Config;

pub struct Metadata {
    pub algorithm: Option<Algorithm>,
    pub key: Key,
    pub duration: i64,
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
        let duration = config.duration();

        Self { algorithm, key, duration }
    }
}

// Default duration is 7 days.
const DEFAULT_DURATION: i64 = 7 * 24 * 60 * 60;

/**
 * JWT config keys in [Config].
 *
 * JWT sign algorithm is [sign_algorithm]
 * Where [sign_algorithm] = "jwt.sign.alg"
 *
 * JWT signing with asymmetric key with RSA or symmetric key.
 *
 * Asymmetric key with RSA requires 2 keys, supporting both PEM and DER format.
 * PEM format requires both [key_rsa_pem_pri] and [key_rsa_pem_pub].
 * Where [key_rsa_pem_pri] = "jwt.key.rsa-pem.pri"
 *       [key_rsa_pem_pub] = "jwt.key.rsa-pem.pub"
 *
 * DER format requires both [key_rsa_der_pri] and [key_rsa_der_pub].
 * Where [key_rsa_der_pri] = "jwt.key.rsa-der"
 *       [key_rsa_der_pub] = "jwt.key.rsa-der"
 *
 * Symmetric key requires [key_secret].
 * Where [key_secret] = "jwt.key.secret"
 *
 * JWT duration is [duration].
 * Where [duration] = "jwt.duration": set as [DEFAULT_DURATION] (7 days) if not specified
 **/
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

    pub fn duration() -> Vec<String> {
        str_vec!["jwt", "duration"]
    }

}

trait MetadataConfig {
    fn algorithm(&self) -> Option<Algorithm>;
    fn key(&self) -> Key;
    fn duration(&self) -> i64;
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

    fn duration(&self) -> i64 {
        self.get(key::duration())
            .map(|duration| duration.parse::<i64>().ok())
            .flatten()
            .unwrap_or_else(|| DEFAULT_DURATION)
    }

}