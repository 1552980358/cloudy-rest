use rocket::{Build, Rocket};

pub trait Rest {
    fn mount_rest(self) -> Self;
}

impl Rest for Rocket<Build> {
    fn mount_rest(self) -> Self {
        self /*TODO: To be implemented*/
    }
}