use std::collections::HashMap;

use super::source::{config_file, env_vars, console_args};

pub trait Loader {
    fn load_config_file(&mut self);
    fn load_env_vars(&mut self);
    fn load_console_args(&mut self);
}

impl Loader for HashMap<String, String> {

    fn load_config_file(&mut self) {
        if let Some(key_value_configs) = config_file() {
            self.extend(key_value_configs);
        }
    }

    fn load_env_vars(&mut self) {
        self.extend(env_vars());
    }

    fn load_console_args(&mut self) {
        self.extend(console_args());
    }

}