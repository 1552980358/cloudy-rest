use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "_id")]
    pub object_id: ObjectId,
    pub username: String,
    pub public_keys: Vec<String>,
    // TODO: To be implemented
}

impl Account {
    // TODO: To be implemented
}