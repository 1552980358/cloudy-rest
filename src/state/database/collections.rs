use mongodb::{Collection, Database};

use super::collection::{Account, Token};

pub struct Collections {
    pub account: Collection<Account>,
    pub token: Collection<Token>,
}

impl Collections {

    pub fn new(database: Database) -> Self {
        Self {
            account: database.collection(collection_name::ACCOUNT),
            token: database.collection(collection_name::TOKEN),
        }
    }

}

mod collection_name {
    pub const ACCOUNT: &str = "account";
    pub const TOKEN: &str = "token";
}