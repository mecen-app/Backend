use rocket::{Build, Rocket, routes};

pub mod model;
pub mod token;

use std::borrow::Borrow;
use rocket::*;
use rocket::http::Status;
use super::db;
use rocket::serde::json::{Json, json, Value};
use uuid::Uuid;
use model::User;
use token::Token;

#[post("/")]
pub async fn get_user_infos(user: User) -> Result<Json<Value>, Status> {
    Ok(Json(json!(user)))
}

#[post("/", rank=2)]
pub async fn create_first_connection(token: Token) -> Result<Json<Value>, Status> {
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
        user_name: "".to_string(),
        mango_pay_user_id: "".to_string(),
        mango_wallet_id: "".to_string()
    };
    let res = user.create_user(db.borrow()).await;
    res.map(|item| item)
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/user", routes![get_user_infos, create_first_connection])
}