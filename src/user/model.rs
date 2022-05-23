#![allow(proc_macro_derive_resolution_fallback)]

use std::borrow::Borrow;
use std::io::ErrorKind;
use mongodb::{sync::Database};
use mongodb::bson::doc;
use mongodb::error::Error;
use rocket_contrib::json::JsonValue;
use serde::{Serialize, Deserialize};
use serde_json::json;


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub token_id: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicUser {
    pub id: String,
    pub email: String,
}

impl User {

    pub fn to_public(user: User) -> PublicUser {
        PublicUser {
            id: user.id,
            email: user.email
        }
    }

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

    pub fn infos(id: String, connection: &Database) -> Result<JsonValue, Error> {
        let result = connection.collection::<User>("users").find_one(doc! {"id": id.to_string()}, None)?;
        match result {
            Some(user) => Ok(JsonValue::from(json!(User::to_public(user)))),
            None => Err(Error::from(ErrorKind::NotFound))
        }
    }

    pub fn create_or_get(&mut self, connection: &Database) -> Result<JsonValue, Error> {
        let user = User::from_email(self.email.to_string(), connection).map(|user| JsonValue::from(json!(user)));
        dbg!(user.borrow());
        match user {
            Some(user) => {
                dbg!(&user);
                return Ok(JsonValue::from(json!(user)))
            },
            None => {
                match connection.collection::<User>("users").insert_one(self.borrow(), None) {
                    Ok(val) => {
                        dbg!(val);
                        Ok(JsonValue::from(json!(self)))
                    },
                    Err(e) => {
                        dbg!(e.borrow());
                        Err(e)
                    }
                }
            }
        }
    }
}