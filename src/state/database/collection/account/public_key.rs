use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PublicKey {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub key: String,
    pub validity: Validity,
}

#[derive(Serialize, Deserialize)]
pub enum Validity {
    Master,
    Permanent,
    Temporary(i64),
    Disabled(i64),
}