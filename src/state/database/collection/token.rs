use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

mod state;
pub use state::State;

#[derive(Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub account: ObjectId,
    pub expiry: i64,
    #[serde(default)]
    #[serde(skip_serializing_if = "State::is_normal")]
    pub state: State,
}

impl Default for State {
    fn default() -> Self {
        Self::Normal
    }
}

impl State {
    fn is_normal(&self) -> bool {
        matches!(self, Self::Normal)
    }
}

impl Token {

    pub fn new(id: ObjectId, account: ObjectId, expiry: i64) -> Self {
        Self {
            id,
            account,
            expiry,
            state: State::Normal,
        }
    }

    pub fn with_passkey(id: ObjectId, account: ObjectId, expiry: i64, passkey: String) -> Self {
        Self {
            id,
            account,
            expiry,
            state: State::Passkey(passkey),
        }
    }

}