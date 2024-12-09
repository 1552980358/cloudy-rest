#[macro_use] extern crate rocket;

#[launch]
async fn rocket() -> _ {
    rocket::build()
}