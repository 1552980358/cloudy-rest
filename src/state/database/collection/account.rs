use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub mod public_key;
pub use public_key::PublicKey;

mod onetime_password_secret;
pub use onetime_password_secret::OnetimePasswordSecret;

#[derive(Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub public_keys: Vec<PublicKey>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onetime_password_secret: Option<OnetimePasswordSecret>,
    // TODO: To be implemented
}

impl Account {
    // TODO: To be implemented
}