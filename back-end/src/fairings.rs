use crate::db::get_database;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Build, Rocket};

pub struct DbFairing;

#[rocket::async_trait]
impl Fairing for DbFairing {
    fn info(&self) -> Info {
        Info {
            name: "MongoDB Connection",
            kind: Kind::Ignite | Kind::Liftoff,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        let _ = get_database().await;
        Ok(rocket)
    }
}

