use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OnetimePasswordSecret {
    pub issue: i64,
    pub secret: String,
}