pub mod google_token;

use std::borrow::Borrow;
use rocket::*;
use rocket::http::Status;
use super::model::User;
use super::super::db;
use rocket::serde::json::Json;
use rocket_contrib::json::JsonValue;
use serde_json::json;
use uuid::Uuid;
use google_token::GoogleToken;

#[post("/")]
pub async fn get_user_infos(user: User) -> Result<Json<JsonValue>, Status> {
    Ok(Json(JsonValue::from(json!(user))))
}

#[post("/", rank=2)]
pub async fn create_first_connection(token: GoogleToken) -> Result<Json<JsonValue>, Status> {
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