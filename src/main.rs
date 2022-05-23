#![feature(plugin)]
#![feature(decl_macro, proc_macro_hygiene)]
extern crate rocket;

extern crate dotenv;
extern crate bcrypt;

#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket::http::Method;
use rocket::Rocket;

mod db;
mod user;

fn rocket() -> Rocket {
    dotenv().ok();
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    let mut rocket = rocket::ignite().attach(cors.to_cors().unwrap());
    rocket = user::mount(rocket);
    rocket
}

fn main() {
    let rocket = rocket();
    rocket.launch();
}