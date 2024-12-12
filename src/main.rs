#[macro_use] extern crate rocket;

mod ext;

mod state;
use state::{Config, Database};

mod rest;
use rest::Rest;

#[launch]
async fn rocket() -> _ {
    let config = Config::load();
    let database = Database::from_config(&config);

    rocket::build()
        .manage(config)
        .manage(database)
        .mount_rest()
}