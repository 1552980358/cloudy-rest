use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum State {
    Normal,
    Passkey(String),
    Disabled(i64),
}