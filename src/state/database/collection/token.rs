use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "_id")]
    pub object_id: ObjectId,
    pub account: ObjectId,
    pub expiry: i64,
}

impl Token {

    pub fn new(object_id: ObjectId, account: ObjectId, expiry: i64) -> Self {
        Self {
            object_id,
            account,
            expiry,
        }
    }

}