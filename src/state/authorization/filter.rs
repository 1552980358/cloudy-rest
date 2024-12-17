use mongodb::bson::{
    oid::ObjectId,
    ser::Error,
    to_document,
    Document
};
use rocket::serde::Serialize;

#[derive(Serialize)]
struct TokenFilter {
    #[serde(rename = "_id")]
    id: ObjectId,
    #[serde(rename = "account")]
    account_id: ObjectId,
    expiry: i64,
}

#[derive(Serialize)]
struct AccountFilter {
    #[serde(rename = "_id")]
    id: ObjectId,
}

pub fn of_token_and_account(token_id: ObjectId, account_id: ObjectId, expiry: &i64) -> Result<(Document, Document), Error> {
    let token_filter = TokenFilter {
        id: token_id, account_id, expiry: *expiry,
    };
    let account_filter = AccountFilter { id: account_id };

    Ok((to_document(&token_filter)?, to_document(&account_filter)?))
}