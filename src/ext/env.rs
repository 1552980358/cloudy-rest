use std::env;
use regex::Regex;

pub fn filter_vars(regex: Regex) -> Vec<(String, String)> {
    env::vars()
        .filter_map(|pair| {
            if regex.is_match(&*pair.0) { Some(pair) }
            else { None }
        })
        .collect()
}

pub fn filter_args(regex: Regex) -> Vec<String> {
    env::args()
        .filter_map(|arg| {
            if regex.is_match(&*arg) { Some(arg) }
            else { None }
        })
        .collect()
}

pub fn filter_arg(regex: Regex) -> Option<String> {
    filter_args(regex).first()
        .cloned()
}