use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

mod state;
pub use state::State;

mod issuer;
pub use issuer::Issuer;

#[derive(Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub account: ObjectId,
    pub expiry: i64,
    pub issuer: Issuer,
    #[serde(default)]
    #[serde(skip_serializing_if = "State::is_normal")]
    pub state: State,
}

impl State {
    fn is_normal(&self) -> bool {
        matches!(self, Self::Normal)
    }
}

impl Token {

    fn new(
        id: ObjectId,
        account: ObjectId,
        expiry: i64,
        issuer: Issuer,
    ) -> Self {
        Self {
            id,
            account,
            issuer,
            expiry,
            state: State::Normal,
        }
    }

    pub fn of_signature(
        id: ObjectId,
        account: ObjectId,
        public_key: ObjectId,
        expiry: i64,
    ) -> Self {
        Self::new(
            id,
            account,
            expiry,
            Issuer::PublicKey(public_key),
        )
    }

}