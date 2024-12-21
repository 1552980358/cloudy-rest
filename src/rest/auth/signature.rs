use chrono::{DateTime, Duration, Utc};
use mongodb::{
    bson,
    bson::{doc, oid::ObjectId}
};
use openssl::{
    base64,
    rsa::{Padding, Rsa},
};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::{
    state::{
        database::collection::Token,
        Config,
        ConfigState,
        DatabaseState,
        JsonWebTokenState
    },
    str_vec,
};

#[derive(Debug, Deserialize)]
struct SignatureRequest {
    usr: String,
    oid: String,
    sig: String,
}

#[derive(Debug, Serialize)]
struct AccountFilter {
    username: String,
}

#[derive(Debug, Serialize)]
struct TokenFilter {
    #[serde(rename = "_id")]
    id: ObjectId,
}

/**
 * Request:
 * ```text
 * POST /auth/signature HTTP/<HTTP-Version>
 * Content-Type: application/json
 * Content-Length: <Length-of-Body>
 *
 * {
 *     "usr": "<Username>",
 *     "oid": "<ObjectId-Hex>",
 *     "sig": "<Signature>"
 * }
 * ```
 *
 * Successful Response:
 * ```text
 * HTTP/<HTTP-Version> 200 OK
 * Content-Type: text/plain
 * Content-Length: <Length-of-Body>
 *
 * <JWT Token String>
 * ```
 *
 * All errors are responded with HTTP status codes only.
 * ```text
 * HTTP/<HTTP-Version> <Error-Status-Code> <Error-Status-Message>
 * ```
 **/
#[post("/signature", data = "<json_request_body>")]
pub async fn verification(
    config: &ConfigState,
    database: &DatabaseState,
    jsonwebtoken: &JsonWebTokenState,
    json_request_body: Json<SignatureRequest>,
) -> Result<String, Status> {
    let signature_request = json_request_body.into_inner();

    let object_id = ObjectId::parse_str(&signature_request.oid)
        .map_err(|_| Status::BadRequest)?;

    let object_id_timestamp = DateTime::from_timestamp_millis(object_id.timestamp().timestamp_millis())
        .ok_or(Status::InternalServerError)?;
    if is_object_id_expired(config, &object_id_timestamp) {
        return Err(Status::BadRequest);
    }

    let account_filter = AccountFilter {
        username: signature_request.usr,
    };
    let filter_document = bson::to_document(&account_filter)
        .map_err(|_| Status::InternalServerError)?;
    let account = database.collections.account
        .find_one(filter_document)
        .await
        // Handle collection filtering / connection error
        .map_err(|_| Status::InternalServerError)?
        // Handle account not found
        .ok_or(Status::NotFound)?;

    let signature = base64::decode_block(signature_request.sig.as_ref())
        .map_err(|_| Status::BadRequest)?;

    let public_key = account.public_keys.iter()
        .find(|public_key| {
            verify_rsa_public_key(&public_key.key, &signature, &signature_request.oid)
                .unwrap_or(false)
        })
        // If no public key is found, return unauthorized
        .ok_or(Status::Unauthorized)?;

    let token_filter = TokenFilter { id: object_id };
    let token_filter = bson::to_document(&token_filter)
        .map_err(|_| Status::InternalServerError)?;
    if database.collections.token.find_one(token_filter)
        .await
        .map_err(|_| Status::InternalServerError)?
        // Make sure the token id is unique
        .is_some() {
        return Err(Status::Conflict);
    }

    let claims = jsonwebtoken.new_claims(
        &object_id.to_hex(), &account.id.to_hex(), &public_key.id.to_hex(), &object_id_timestamp,
    );
    let jwt_str = jsonwebtoken
        .encode_jwt(&claims)
        .map_err(|_| Status::InternalServerError)?;

    let token = Token::new(object_id, account.id, public_key.id, claims.expiry);
    let inserted_object_id = database.collections.token
        .insert_one(token)
        .await
        // Handle driver error
        .map_err(|_| Status::InternalServerError)?
        .inserted_id
        .as_object_id()
        // Handle object id conversion error
        .ok_or(Status::InternalServerError)?;
    // Make sure the inserted object id is the same as the object id
    if inserted_object_id.to_hex() != object_id.to_hex() {
        return Err(Status::InternalServerError);
    }

    Ok(jwt_str)
}

// 30 seconds
const OID_TIMEOUT: i64 = 30 * 1000;

trait Signature {
    fn oid_timeout_millis(&self) -> i64;
}

impl Signature for Config {
    fn oid_timeout_millis(&self) -> i64 {
        self.get(str_vec!["auth", "signature", "oid-timeout"])
            .map(|timeout| timeout.parse::<i64>().ok())
            .flatten()
            .unwrap_or(OID_TIMEOUT)
    }
}

fn is_object_id_expired(config: &Config, object_id_timestamp: &DateTime<Utc>) -> bool {
    let now_timestamp = Utc::now();
    let valid_duration = config.oid_timeout_millis();

    // object_id is from the future or too old || object_id expired
    *object_id_timestamp > now_timestamp ||
        (*object_id_timestamp + Duration::milliseconds(valid_duration) < now_timestamp)
}

fn verify_rsa_public_key(
    public_key: &String,
    signature: &Vec<u8>,
    object_id_hex: &String,
) -> Result<bool, ()> {
    let rsa_public = Rsa::public_key_from_pem(public_key.as_ref()).map_err(|_| ())?;
    let mut decryption_buffer = vec![0; rsa_public.size() as usize];
    let decrypted_len = rsa_public
        .public_decrypt(signature.as_ref(), &mut decryption_buffer, Padding::PKCS1)
        .map_err(|_| ())?;
    decryption_buffer.truncate(decrypted_len);
    let decrypted_signature = String::from_utf8(decryption_buffer).map_err(|_| ())?;
    Ok(&*decrypted_signature == *object_id_hex)
}

#[cfg(test)]
mod test {
    use super::verify_rsa_public_key;
    use openssl::base64;

    // noinspection SpellCheckingInspection
    #[test]
    fn test_verify_rsa_public_key_pem() {
        let object_id_hex = "675f21efdbd4c628b5e9496a".to_string();

        let public_key = "\
                -----BEGIN PUBLIC KEY-----\n\
                MIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEA1LzworQVsh6WEQ8pFIpH\n\
                q3llXMd9frJcJwKcJFwIBlC1rlFnoDSAW4CK0F22BcPcp0AkAwFvD2esdxmA1lHZ\n\
                zOG9qhefwzHhFPLhlFP62FQL88qHGPh1jqrsQuAdQA0Xmj2uX4ZWptashPx7cpyK\n\
                o395hrr7joe4Bxs3EMDJDoWlDWmRZxN9uHnxpKD5lqJRDfAr11yGwW1RBRosQUbz\n\
                n190FzlLlJLGm99cCiIRMYZ8Pja2Jq9x68se7N1nQPb3vdotNR7KSzlPdcVrB83c\n\
                ozo4UzJfIdXKiscgrgSPUtu/eL4QQMQe8K/b1ZniEhMCs8bSBrBSH9veUJqeV0/i\n\
                DifQxR0qCFbHattFvJwDWvpPlJweq65TzKnwYtUoOig9JC/RuUQFpJJcjqsMMZZy\n\
                nYBTZJVVgQprnhN7dkvT9bHzCZnVeZkVFgU/6+1/sXwMBSQJA/dytUzuU3fC+W3U\n\
                UdrdyGtgwmSUGHfT1gbFGlXRBqb/rthqRuLyW5j1uQOr66gWjCGMBsru1mJM5Qa6\n\
                HwxdgLv/zhOJuYby6t+3AVmi1cQVLV3KgNn/+CCYSUllWC0gOIoYH4qWletiHUJL\n\
                sl0hBYDBOnDPQv9VBbhuLpAOd+gLDrgDHvR65RDg6j/lke8uh5O/tfQygIMVYO8B\n\
                AblnepcttBeKs9urwWxBkX0CAwEAAQ==\n\
                -----END PUBLIC KEY-----\n\
                "
        .to_string();
        // echo -n "675f21efdbd4c628b5e9496a" | openssl pkeyutl -sign -inkey <PrivateKeyPath> | openssl base64
        let signature = "\
                piZXX6AsES6AQDdV12yK0d1SYvQeO/grrxNdSsIaev63ITvToCd9dZgFH/7TkuDt\
                DSHXGd7hfkKX4CMJBX0gEabFQh2yk878IX/FFEjZFdOxaRA78MZUULHHzt3+c9VK\
                Zisx2h8OJDIkA/JrktazK5HDlMVSRb4HhZF0AzxAVLN0k1e9GhZyVYFwxYf9HgeT\
                maDUm/s6jp/QcRzdBY/hE/1VW6IAJ1xTJ9rZ13/Q/tqRsUvv8p7wUfHmrbgX8kFP\
                xjHwOKl7d/zpZhowCuDY4DsQI4bYJJ0mgyVfI5v3EPp18gsOY9lREb8UOMpL8hyx\
                9oQTa119YthX8TbZRfRLkFfpNsTGpTRXQ3b5DrsnHBQvFcUdEtfwgb8Jc55l4M/f\
                F8jegneo4K34uVENr5B7qgTf77zO0QP/8M/kKToKf9p9Fv4bwGdckBdpa/H+Ak5G\
                oOTjnpUZaY0TAA+7o4puMHzWb8bdOKk+tu8KgGBkLvKmZCFXdeQpDi8PPtRUnimE\
                oQ/FSp+2n2xksX+EnlRxzNAOnYcr9y7pAaFt2l8alLlTeHD7FS3iqWaERIyh0NFP\
                GJP7JPqyC8VRdZUdx8ouXXrcRxsV2WBjD/7PLb1aVh19KSoMj72YGQkPcZuPIqfu\
                m7Y6KTYPxRvNvmSEQ+ANStAPqm3bjjy8GGPu4UNES00=\
                ";
        let signature_bytes = base64::decode_block(signature).unwrap();

        verify_rsa_public_key(&public_key, &signature_bytes, &object_id_hex)
            .map(|is_valid| assert!(is_valid))
            .unwrap_or_else(|_| panic!("Invalid RSA public key verification"));
    }
}
