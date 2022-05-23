#![allow(proc_macro_derive_resolution_fallback)]

use std::borrow::Borrow;
use jwks_client::keyset::KeyStore;
use mongodb::{sync::Database};
use mongodb::bson::{Array, doc};
use rocket::{Request, request};
use rocket::request::FromRequest;
use rocket_contrib::json::JsonValue;
use serde::{Serialize, Deserialize};
use serde_json::json;
use rocket::http::Status;
use rocket::outcome::Outcome;
use crate::db;


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub token_id: String,
    pub email: String,
    pub balance: i32,
    pub friends: Array,
    pub open_loans: Array,
    pub open_borrows: Array,
    pub open_propositions: Array,
    pub user_name: String
}

impl User {

    pub fn from_email(email: String, connection: &Database) -> Option<User> {
        let result = connection.collection::<User>("users").find_one(doc! {"email": email}, None);
        match result {
            Ok(v) => Some(v?),
            Err(e) => {
                dbg!(e);
                None
            }
        }
    }

    pub fn set_google_account(&mut self) -> Result<&mut User, jwks_client::error::Error> {
        let jkws_url = "https://www.googleapis.com/oauth2/v3/certs";
        let key_set = KeyStore::new_from(jkws_url).unwrap();

        match key_set.verify(self.token_id.borrow()) {
            Ok(jwt) => {
                self.user_name = jwt.payload().get_str("name").unwrap().to_string();
                self.email = jwt.payload().get_str("email").unwrap().to_string();
                Ok(self)
            }
            Err(e) => {
                dbg!(e.borrow());
                Err(e)
            }
        }
    }

    pub fn get_user_from_token_google(mut token: String, connection: &Database) -> Option<User> {
        let jkws_url = "https://www.googleapis.com/oauth2/v3/certs";
        let key_set = KeyStore::new_from(jkws_url).unwrap();

        token = token.replace("Bearer ", "");
        match key_set.verify(token.as_str()) {
            Ok(jwt) => {
                dbg!(jwt.payload());
                User::from_email(jwt.payload().get_str("email").unwrap().to_string(), connection)
            }
            Err(e) => {
                dbg!(e.borrow());
                None
            }
        }
    }

    pub async fn create_or_get_google(&mut self, connection: &Database) -> Result<JsonValue, Status> {
        self.set_google_account().map_err(|_| Status::Unauthorized)?;
        match connection.collection::<User>("users").insert_one(self.borrow(), None) {
            Ok(val) => {
                dbg!(val);
                Ok(JsonValue::from(json!(self)))
            },
            Err(e) => {
                dbg!(e.borrow());
                Err(Status::AlreadyReported)
            }
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        let db = match db::connect() {
            Ok(db) => db,
            Err(e) => {
                dbg!(e);
                return Outcome::Forward(())
            }
        };
        match User::get_user_from_token_google(keys[0].to_string(), &db) {
            Some(user) => Outcome::Success(user),
            None => Outcome::Forward(())
        }
    }
}

pub(crate) struct GoogleToken {
    pub value: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GoogleToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, ()> {
        let token = request.headers().get_one("Authorization");
        match token {
            Some(token) => {
                Outcome::Success(GoogleToken{value: token.replace("Bearer ", "").to_string()})
            },
            None => Outcome::Failure((Status::BadRequest, ()))
        }
    }
}