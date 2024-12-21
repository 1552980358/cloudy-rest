use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub mod public_key;
pub use public_key::PublicKey;

#[derive(Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub public_keys: Vec<PublicKey>,
    // TODO: To be implemented
}

impl Account {
    // TODO: To be implemented
}