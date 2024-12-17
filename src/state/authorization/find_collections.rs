use mongodb::bson::Document;
use tokio::try_join;
use crate::state::{
    database::collection::{Account, Token},
    Database,
};

#[async_trait]
pub trait FindCollections {
    type R;

    async fn find_account_token(&self, filter_documents: (Document, Document)) -> Self::R;
}

#[async_trait]
impl FindCollections for Database {
    type R = Result<(Option<Token>, Option<Account>), mongodb::error::Error>;

    async fn find_account_token(&self, filter_documents: (Document, Document)) -> Self::R {
        let (token_filter, account_filter) = filter_documents;
        try_join!(
            self.collections.token.find_one(token_filter),
            self.collections.account.find_one(account_filter)
        )
    }
}