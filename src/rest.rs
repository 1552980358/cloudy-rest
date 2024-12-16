use rocket::{Build, Rocket};

mod auth;

pub trait Rest {
    fn mount_rest(self) -> Self;
}

impl Rest for Rocket<Build> {
    fn mount_rest(self) -> Self {
        self.mount(auth::MOUNT_POINT, auth::routes())
    }
}