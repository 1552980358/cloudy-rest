use rocket::State;

mod config;
pub use config::Config;
pub type ConfigState = State<Config>;
