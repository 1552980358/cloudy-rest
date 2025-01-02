use rocket::Route;

mod signature;

mod onetime_password;

pub const MOUNT_POINT: &str = "/auth";

pub fn routes() -> Vec<Route> {
    routes![
        // POST /auth/sig
        signature::verify,
        // POST /auth/otp
        onetime_password::verify,
    ]
}