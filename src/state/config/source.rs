use std::collections::HashMap;
use crate::ext::env;
use super::{regex, symbol};

pub fn config_file() -> Option<HashMap<String, String>> {
    let arg_file_regex = regex::arg_file();
    // Check if config file argument is present
    if let Some(config_file_arg) = env::filter_arg(&arg_file_regex) {
        // Extract file path from argument
        if let Some(file_path_captures) = arg_file_regex.captures(&config_file_arg) {
            // Extract file path from captures
            let (_, [file_path]) = file_path_captures.extract();
            if let Some(file_lines) = file_path.read_file_lines() {
                let key_value_configs = file_lines.iter()
                    .filter_map(importer::from_file_line)
                    .collect::<HashMap<String, String>>();
                return Some(key_value_configs);
            }
        }
    }
    None
}

pub fn env_vars() -> HashMap<String, String> {
    env::filter_vars(&regex::env_var())
        .iter()
        .filter_map(importer::from_env_var)
        .collect()
}

pub fn console_args() -> HashMap<String, String> {
    env::filter_args(&regex::console_arg())
        .iter()
        .filter_map(importer::from_console_arg)
        .collect()
}

mod importer {
    use super::{regex, symbol};

    pub fn from_file_line(line: &String) -> Option<(String, String)> {
        match line {
            _ if line.trim().starts_with(symbol::HASH) => { None },
            _ => {
                regex::file_line()
                    .captures(&line)
                    .map(regex::extraction::dot_key_value)
            }
        }
    }

    pub fn from_env_var(key_value: &(String, String)) -> Option<(String, String)> {
        let (key, value) = key_value;
        regex::env_var()
            .captures(key)
            .map(regex::extraction::underline_key)
            .map(|key| (key, value.trim().to_string()))
    }

    pub fn from_console_arg(arg: &String) -> Option<(String, String)> {
        regex::console_arg()
            .captures(arg)
            .map(regex::extraction::dot_key_value)
    }

    #[cfg(test)]
    mod test {
        use super::{from_console_arg, from_env_var, from_file_line};

        #[test]
        fn test_from_file_line() {
            let test_line = "       Test.Key =      TestValue      ".to_string();
            let Some((key, val)) = from_file_line(&test_line) else {
                panic!("Panic: Failed to extract key-value from file line.");
            };
            assert_eq!(key, "test.key", "Key={}", key);
            assert_eq!(val, "TestValue", "Value={}", val);
        }

        #[test]
        fn test_from_env_var() {
            let test_key = "CLOUDY_TEST_KEY".to_string();
            let test_val = "TestValue".to_string();
            let Some((result_key, _)) = from_env_var(&(test_key.clone(), test_val.clone())) else {
                panic!("Panic: Failed to extract key-value from env var.");
            };
            assert_eq!(result_key, "test.key", "Result={}", result_key);
        }

        #[test]
        fn test_from_console_arg() {
            let test_arg = r#"-c" TEST.key =  TestValue ""#;
            let Some((key, val)) = from_console_arg(&test_arg.to_string()) else {
                panic!("Panic: Failed to extract key-value from console arg.");
            };
            assert_eq!(key, "test.key", "Key={}", key);
            assert_eq!(val, "TestValue", "Value={}", val);
        }

    }

}

trait FilePath {
    fn read_file_lines(&self) -> Option<Vec<String>>;
}
impl FilePath for &str {
    fn read_file_lines(&self) -> Option<Vec<String>> {
        use std::fs;
        fs::read_to_string(self)
            .map(|content| {
                content.lines()
                    .map(&str::to_string)
                    .collect()
            })
            .ok()
    }
}