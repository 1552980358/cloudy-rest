#[macro_use] extern crate rocket;

mod ext;
mod state;

#[launch]
async fn rocket() -> _ {
    rocket::build()
}