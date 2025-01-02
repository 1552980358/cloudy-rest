use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Issuer {
    OnetimePassword,
    PublicKey(ObjectId),
}