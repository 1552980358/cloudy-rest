use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum State {
    Normal,
    Disabled(i64),
}

impl Default for State {
    fn default() -> Self {
        Self::Normal
    }
}