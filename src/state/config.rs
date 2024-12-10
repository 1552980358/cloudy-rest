use std::collections::HashMap;
use std::ops::Index;

mod regex;

mod source;

mod loader;
use loader::Loader;

mod symbol;

pub struct Config {
    key_value_map: HashMap<String, String>
}

impl Config {

    pub fn load() -> Self {
        let mut key_value_map = HashMap::new();
        key_value_map.load_config_file();
        key_value_map.load_env_vars();
        key_value_map.load_console_args();

        Self { key_value_map }
    }

    fn process_index(index: Vec<String>) -> String {
        index.iter()
            .map(|schema| schema.to_lowercase())
            .collect::<Vec<String>>()
            .join(symbol::INDEX)
    }

    pub fn contains(&self, schemas: Vec<String>) -> bool {
        self.key_value_map
            .contains_key(&Self::process_index(schemas))
    }

    pub fn get(&self, schemas: Vec<String>) -> Option<&String> {
        self.key_value_map
            .get(&Self::process_index(schemas))
    }

}

impl Index<Vec<String>> for Config {

    type Output = String;

    fn index(&self, schemas: Vec<String>) -> &Self::Output {
        &self.key_value_map[&Self::process_index(schemas)]
    }

}