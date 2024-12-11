use rocket::State;

mod config;
pub use config::Config;
pub type ConfigState = State<Config>;

mod database;
pub use database::Database;
pub type DatabaseState = State<Database>;