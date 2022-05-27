#![allow(proc_macro_derive_resolution_fallback)]

use std::borrow::Borrow;
use jwks_client::keyset::KeyStore;
use mangopay::Mangopay;
use mangopay::user::CreateUserBody;
use mangopay::wallet::Wallet;
use rocket::serde::json::{Json, json, Value};
use mongodb::{sync::Database};
use mongodb::bson::{Array, doc};
use rocket::{Request, request};
use rocket::request::FromRequest;
use rocket::serde::{Serialize, Deserialize};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rustc_serialize::json::ToJson;
use crate::db;


#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: String,
    pub token_id: String,
    pub email: String,
    pub balance: i32,
    pub friends: Array,
    pub open_loans: Array,
    pub open_borrows: Array,
    pub open_propositions: Array,
    pub user_name: String,
    pub mango_pay_user_id: String,
    pub mango_wallet_id: String
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

    pub fn set_oauth_account(&mut self) -> Result<&mut User, jwks_client::error::Error> {
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

    pub fn get_user_from_token(mut token: String, connection: &Database) -> Option<User> {
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

    pub fn set_mango_user(&mut self) -> Result<&mut User, Status> {
        let mango: Mangopay = Mangopay::init(env!("MANGO_CLIENT_ID").parse().unwrap(), env!("MANGO_API_KEY").parse().unwrap());
        let user_infos = CreateUserBody {
            first_name: self.user_name.split(' ')[0],
            last_name: self.user_name.split(' ')[1],
            email: self.email.to_string(),
            user_category: "Payer".to_string(),
            tag: "Backend".to_string(),
            terms_and_conditions_accepted: true
        };
        let user = match mango.create_user(&user_infos) {
            Some(user) => user,
            None => return Err(Status::FailedDependency)
        };
        self.mango_pay_user_id = user.id;
        let wallet: Wallet = match mango.create_wallet(self.mango_pay_user_id.to_string()) {
          Some(wallet) => wallet,
            None => return Err(Status::FailedDependency)
        };
        self.mango_wallet_id = wallet.id;
        Ok(self)
    }

    pub async fn create_user(&mut self, connection: &Database) -> Result<Json<Value>, Status> {
        self.set_oauth_account().map_err(|_| Status::Unauthorized)?;
        self.set_mango_user()?;
        match connection.collection::<User>("users").insert_one(self.borrow(), None) {
            Ok(val) => {
                dbg!(val);
                Ok(Json(json!(self)))
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
        match User::get_user_from_token(keys[0].to_string(), &db) {
            Some(user) => Outcome::Success(user),
            None => Outcome::Forward(())
        }
    }
}