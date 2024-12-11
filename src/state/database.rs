use super::Config;

mod connector;

mod metadata;
use metadata::Metadata;

mod collection;
use collection::Collections;

pub struct Database {
    metadata: Metadata,
    pub collections: Collections,
}

impl Database {

    pub fn from_config(config: &Config) -> Self {
        let metadata = Metadata::from_config(config);
        let client = connector::with_metadata(&metadata);
        let collections = Collections::initialize(&client);

        Self { metadata, collections }
    }

}

