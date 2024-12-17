use mongodb::bson::oid::{ObjectId, Error};

use crate::state::jsonwebtoken::Claims;

pub trait ClaimsObjectIds {
    type R;

    fn token_and_account(&self) -> Self::R;
}

impl ClaimsObjectIds for Claims {
    type R = Result<(ObjectId, ObjectId), Error>;

    fn token_and_account(&self) -> Self::R {
        Ok((ObjectId::parse_str(&self.account)?, ObjectId::parse_str(&self.id)?))
    }
}