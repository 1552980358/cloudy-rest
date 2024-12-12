use regex::{Captures, Regex};
use super::symbol;

pub fn arg_file() -> Regex {
    Regex::new(r#"^-C"(.+)"$"#)
        .unwrap_or_else(|_| panic!("Panic: Failed to create regex for config file arg."))
}

pub fn file_line() -> Regex {
    Regex::new(r#"^ *([a-zA-Z][a-zA-Z\-.]+[a-zA-Z]) *= *(.+) *$"#)
        .unwrap_or_else(|_| panic!("Panic: Failed to create regex for config file line."))
}

pub fn env_var() -> Regex {
    Regex::new("^CLOUDY_([a-zA-Z][a-zA-Z_]+[a-zA-Z])$")
        .unwrap_or_else(|_| panic!("Panic: Failed to create regex for config env var."))
}

pub fn console_arg() -> Regex {
    Regex::new(r#"-c" *([a-zA-Z][a-zA-Z\-.]+[a-zA-Z]) *= *(.+) *""#)
        .unwrap_or_else(|_| panic!("Panic: Failed to create regex for console arg line."))
}

pub mod extraction {
    use super::{symbol, Captures};

    pub fn underline_key(captures: Captures) -> String {
        let (_, [key]) = captures.extract();
        key.replace(symbol::DOUBLE_UNDERLINE, symbol::HYPHEN)
            .split(symbol::UNDERLINE)
            .map(&str::to_lowercase)
            .collect::<Vec<String>>()
            .join(symbol::INDEX)
    }

    pub fn dot_key_value(captures: Captures) -> (String, String) {
        let (_, [key, value]) = captures.extract();
        let key = key.split(symbol::DOT)
            .map(&str::to_lowercase)
            .collect::<Vec<String>>()
            .join(symbol::INDEX);

        (key, value.trim().to_string())
    }

}