use rocket::State;

mod authorization;
pub use authorization::Authorization;

mod config;
pub use config::Config;
pub type ConfigState = State<Config>;

pub mod database;
pub use database::Database;
pub type DatabaseState = State<Database>;

mod jsonwebtoken;
pub use jsonwebtoken::JsonWebToken;
pub type JsonWebTokenState = State<JsonWebToken>;