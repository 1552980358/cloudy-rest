use rocket::{
    http::Status,
    Request,
    request::{FromRequest, Outcome},
};
use serde::Serialize;

use super::{
    database::collection::{Account, Token},
    Database,
    JsonWebToken,
};

mod claims;
use claims::ClaimsObjectIds;

mod filter;

mod find_collections;
use find_collections::FindCollections;

pub struct Authorization {
    pub token: Token,
    pub account: Account,
}

#[async_trait]
impl<'r> FromRequest<'r> for Authorization {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let jsonwebtoken = request.rocket().state::<JsonWebToken>().unwrap();
        let database = request.rocket().state::<Database>().unwrap();

        let Some(authorization) = request.authorization() else {
            return Outcome::Error((Status::BadRequest, ()))
        };

        let Ok(claims) = jsonwebtoken.decode_jwt(&authorization) else {
            return Outcome::Error((Status::Unauthorized, ()))
        };
        let Ok((token_id, account_id)) = claims.token_and_account() else {
            return Outcome::Error((Status::InternalServerError, ()))
        };

        let Ok(filter_documents) = filter::of_token_and_account(token_id, account_id, &claims.expiry) else {
            return Outcome::Error((Status::InternalServerError, ()))
        };

        let Ok((token, account)) = database.find_account_token(filter_documents).await else {
            return Outcome::Error((Status::InternalServerError, ()))
        };

        let Some((token, account)) = token.zip(account) else {
            return Outcome::Error((Status::Unauthorized, ()))
        };

        Outcome::Success(Self { token, account })
    }
}

trait AuthorizationHeader {
    fn authorization(&self) -> Option<String>;
}

impl AuthorizationHeader for Request<'_> {
    fn authorization(&self) -> Option<String> {
        self.headers()
            .get_one("Authorization")
            .map(&str::to_string)
    }
}

