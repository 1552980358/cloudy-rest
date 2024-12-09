use std::env;
use regex::Regex;

pub fn filter_vars(regex: Regex) -> Vec<(String, String)> {
    env::vars()
        .filter_map(|key, val| {
            if regex.is_match(&*key) { Some((key, val)) }
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