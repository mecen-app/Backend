extern crate dotenv;
extern crate rocket;

#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Build, Request, Response, Rocket};
use rocket_sentry::RocketSentry;

mod card;
mod db;
mod mangopay;
mod user;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses.",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        println!("Setting access control allow origin");
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

fn rocket() -> Rocket<Build> {
    dotenv().ok();

    let mut rocket = rocket::build().attach(CORS).attach(RocketSentry::fairing());
    rocket = user::mount(rocket);
    rocket = card::mount(rocket);
    rocket
}

#[rocket::main]
async fn main() -> () {
    let rocket = rocket();
    rocket.launch().await;
}
