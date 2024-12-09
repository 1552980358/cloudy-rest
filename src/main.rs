#[macro_use] extern crate rocket;

mod ext;

#[launch]
async fn rocket() -> _ {
    rocket::build()
}