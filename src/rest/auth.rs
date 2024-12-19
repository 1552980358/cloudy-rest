use rocket::Route;

mod signature;

pub const MOUNT_POINT: &str = "/auth";

pub fn routes() -> Vec<Route> {
    routes![
        // POST /auth/signature
        signature::verification,
    ]
}