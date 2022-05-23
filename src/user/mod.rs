use rocket::{Build, Rocket, routes};

pub mod model;
pub mod google;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/user/google", routes![google::get_user_infos, google::create_first_connection])
}