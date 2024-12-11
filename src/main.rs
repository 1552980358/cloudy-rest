#[macro_use] extern crate rocket;

mod ext;
mod state;
use state::{Config, Database};

#[launch]
async fn rocket() -> _ {
    let config = Config::load();
    let database = Database::from_config(&config);

    rocket::build()
        .manage(config)
        .manage(database)
}