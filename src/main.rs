extern crate rocket;

extern crate dotenv;

extern crate serde_derive;

use dotenv::dotenv;
use rocket::http::Header;
use rocket::{Build, Request, Response, Rocket};
use rocket::fairing::{Fairing, Info, Kind};

mod db;
mod user;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        println!("Setting access control allow origin");
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

    }
}

fn rocket() -> Rocket<Build> {
    dotenv().ok();

    let mut rocket = rocket::build().attach(CORS);
    rocket = user::mount(rocket);
    rocket
}

#[rocket::main]
async fn main() -> () {
    let rocket = rocket();
    rocket.launch().await;
}