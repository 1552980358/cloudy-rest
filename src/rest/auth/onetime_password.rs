#![allow(private_interfaces)]
use chrono::DateTime;
use mongodb::bson::oid::ObjectId;
use openssl::hash::MessageDigest;
use rocket::{
    http::Status,
    serde::json::Json,
};
use serde::Deserialize;

use crate::state::{
    database::collection::Token,
    Config, ConfigState, DatabaseState, JsonWebTokenState
};
use crate::str_vec;

#[derive(Deserialize)]
struct VerifyOtpRequest {
    pub usr: String,
    pub otp: String,
}

#[post("/otp", data = "<json_request_body>")]
pub(super) async fn verify(
    config: &ConfigState,
    database: &DatabaseState,
    jsonwebtoken: &JsonWebTokenState,
    json_request_body: Json<VerifyOtpRequest>,
) -> Result<String, Status> {
    let verify_otp_request = json_request_body.into_inner();

    let filter = account_filter::from_username(&verify_otp_request.usr)
        .map_err(|_| Status::InternalServerError)?;
    let account = database.collections.account.find_one(filter)
        .await
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::Forbidden)?;
    let otp_secret = account.onetime_password_secret
        .ok_or(Status::Forbidden)?;

    let message_digest = config.hashing_algorithm();
    let hash = otp::hash(&otp_secret.secret, message_digest)
        .map_err(|_| Status::InternalServerError)?;
    let otp = otp::generate(hash)
        .ok_or(Status::InternalServerError)?;
    if otp != verify_otp_request.otp {
        return Err(Status::Unauthorized);
    }

    let object_id = ObjectId::new();
    let timestamp = DateTime::from_timestamp(object_id.timestamp().timestamp_millis(), 0)
        .ok_or(Status::InternalServerError)?;
    let claims = jsonwebtoken.new_claims(&object_id.to_hex(), &account.id.to_hex(), &timestamp);
    let token = Token::of_passkey(object_id, account.id, claims.expiry);
    let inserted_id = database.collections.token.insert_one(&token)
        .await
        .map_err(|_| Status::InternalServerError)?
        .inserted_id
        .as_object_id()
        .ok_or(Status::InternalServerError)?;
    if inserted_id.to_hex() != object_id.to_hex() {
        return Err(Status::InternalServerError);
    }
    let jwt = jsonwebtoken.encode_jwt(&claims)
        .map_err(|_| Status::InternalServerError)?;
    Ok(jwt)
}

mod account_filter {
    use mongodb::{
        bson,
        bson::{
            ser::Result,
            Document,
        },
    };
    use serde::Serialize;

    #[derive(Serialize)]
    struct Filter {
        pub username: String,
    }

    pub fn from_username(username: &String) -> Result<Document> {
        let filter = Filter { username: username.clone() };
        bson::to_document(&filter)
    }

}

trait OnetimePassword {
    fn hashing_algorithm(&self) -> MessageDigest;
}

const OTP_HASHING_ALGORITHM: &str = "SHA256";
impl OnetimePassword for Config {
    fn hashing_algorithm(&self) -> MessageDigest {
        let algorithm = self.get(str_vec!["auth", "otp", "hash-alg"])
            .map(|algorithm| algorithm.to_uppercase())
            .unwrap_or(OTP_HASHING_ALGORITHM.into());
        match algorithm.as_str() {
            "MD5" => MessageDigest::md5(),

            /* SHA-1 */
            "SHA1" => MessageDigest::sha1(),

            /* SHA-3 Family */
            "SHA224" => MessageDigest::sha224(),
            "SHA384" => MessageDigest::sha384(),
            "SHA512" => MessageDigest::sha512(),

            /* SHA-3 Family */
            "SHA3-224" => MessageDigest::sha3_224(),
            "SHA3-256" => MessageDigest::sha3_256(),
            "SHA3-384" => MessageDigest::sha3_384(),
            "SHA3-512" => MessageDigest::sha3_512(),

            OTP_HASHING_ALGORITHM | _ => MessageDigest::sha256(),
        }
    }
}

mod otp {
    use chrono::Utc;
    use openssl::{
        error::ErrorStack,
        hash::{DigestBytes, Hasher, MessageDigest},
        base64,
    };

    const TIMEOUT: i64 = 30/*seconds*/;
    const DIGITS: u32 = 6;

    pub fn hash(
        secret: &String,
        message_digest: MessageDigest,
    ) -> Result<DigestBytes, ErrorStack> {
        let secret_bytes = base64::decode_block(&*secret)?;
        let time_counter = (Utc::now().timestamp() / TIMEOUT).to_be_bytes();

        let mut hasher = Hasher::new(message_digest)?;
        hasher.update(&secret_bytes)?;
        hasher.update(&time_counter)?;

        hasher.finish()
    }

    pub fn generate(
        digest_bytes: DigestBytes,
    ) -> Option<String> {
        let Some(offset) = digest_bytes.last().map(|byte| (byte & 0xF) as usize) else {
            return None;
        };

        let binary_code = extract_information(offset).iter()
            .map(|information| digest_bytes.extract(*information))
            .fold(0, |acc, val| acc | val);

        let totp = binary_code % 10_u32.pow(DIGITS);
        let totp = format!("{:0digits$}", totp, digits = DIGITS as usize);
        Some(totp)
    }

    fn extract_information(offset: usize) -> Vec<(usize, u32, u32)> {
        vec![
            (offset, 0x7F, 24),
            (offset + 1, 0xFF, 16),
            (offset + 2, 0xFF, 8),
            (offset + 3, 0xFF, 0)
        ]
    }

    trait Hashing {
        fn extract(&self, information: (usize, u32, u32)) -> u32;
    }

    impl Hashing for DigestBytes {
        fn extract(&self, information: (usize, u32, u32)) -> u32 {
            let (index, mask, shifting) = information;
            (self[index] as u32 & mask) << shifting
        }
    }

}