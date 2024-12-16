use super::Config;

mod connector;

mod metadata;
use metadata::Metadata;

pub mod collection;

mod collections;
use collections::Collections;

pub struct Database {
    metadata: Metadata,
    pub collections: Collections,
}

impl Database {

    pub fn from_config(config: &Config) -> Self {
        let metadata = Metadata::from_config(config);
        let client = connector::with_metadata(&metadata);
        let database = client.database(metadata.db_name.as_str());
        let collections = Collections::new(database);

        Self {
            metadata,
            collections,
        }
    }

}

