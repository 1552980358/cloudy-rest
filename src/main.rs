#[macro_use] extern crate rocket;

mod ext;
mod state;
use state::{Config};

#[launch]
async fn rocket() -> _ {
    let config = Config::load();

    rocket::build()
        .manage(config)
}