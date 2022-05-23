use std::borrow::Borrow;
use rocket::*;
use rocket::http::Status;
use super::model::{User, GoogleToken};
use super::super::db;
use rocket::serde::json::Json;
use rocket_contrib::json::JsonValue;
use serde_json::json;
use uuid::Uuid;

#[post("/google")]
async fn get_user_infos(user: User) -> Result<Json<JsonValue>, Status> {
    Ok(Json(JsonValue::from(json!(user))))
}

#[post("/google", rank=2)]
async fn create_first_connection(token: GoogleToken) -> Result<Json<JsonValue>, Status> {
    let db = match db::connect() {
        Ok(db) => db,
        Err(e) => {
            dbg!(e);
            return Err(Status::FailedDependency)
        }
    };
    let mut user = User {
        id: Uuid::new_v4().to_string(),
        token_id: token.value,
        email: "".to_string(),
        balance: 0,
        friends: vec![],
        open_loans: vec![],
        open_borrows: vec![],
        open_propositions: vec![],
        user_name: "".to_string()
    };
    let res = user.create_or_get_google(db.borrow()).await;
    res.map(|item| Json(item))
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/user", routes![get_user_infos, create_first_connection])
}