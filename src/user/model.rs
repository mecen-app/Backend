use crate::db;
use crate::mangopay;
use crate::mangopay::model::Mangopay;
use jwks_client::keyset::KeyStore;
use mangopay::user::CreateUserBody;
use mangopay::wallet::{CreateWallet, Wallet};
use mongodb::bson;
use mongodb::bson::{doc, Array, Bson};
use mongodb::sync::Database;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::{request, Request};
use std::borrow::Borrow;
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: String,
    pub token_id: String,
    pub email: String,
    pub balance: i64,
    pub friends: Array,
    pub open_loans: Array,
    pub open_borrows: Array,
    pub open_propositions: Array,
    pub user_name: String,
    pub mango_pay_user_id: String,
    pub mango_pay_wallet_id: String,
}

impl User {
    pub async fn from_email(email: String, connection: &Database) -> Option<User> {
        let result = connection
            .collection::<User>("users")
            .find_one(doc! {"email": email}, None);
        match result {
            Ok(v) => {
                let mut user: User = v?;
                user.get_wallet_balance(connection).await;
                Some(user)
            }
            Err(e) => {
                dbg!(e);
                None
            }
        }
    }

    pub fn update_user(&self, connection: &Database) -> Result<User, Status> {
        let result = connection.collection::<User>("users").find_one_and_update(
            doc! {"id": self.id.clone()},
            doc! {"$set": self},
            None,
        );
        match result {
            Ok(v) => Ok(v.unwrap()),
            Err(e) => {
                dbg!(e);
                Err(Status::FailedDependency)
            }
        }
    }

    pub async fn get_wallet_balance(&mut self, _connection: &Database) -> i64 {
        let mango: Mangopay = Mangopay::init(
            env::var("MANGO_CLIENT_ID")
                .expect("MANGO_CLIENT_ID not set")
                .parse()
                .unwrap(),
            env::var("MANGO_API_KEY")
                .expect("MANGO_API_KEY not set")
                .parse()
                .unwrap(),
            "https://api.sandbox.mangopay.com/v2.01/".to_string(),
        );

        let mangouser = mango.list_wallets(self.mango_pay_user_id.to_string()).await;
        match mangouser {
            Ok(v) => {
                let mut res: i64 = 0;
                for w in v.iter() {
                    res += w.balance.amount;
                }
                self.balance = res;
                res
            }
            Err(_e) => 0,
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

    pub async fn get_user_from_token(mut token: String, connection: &Database) -> Option<User> {
        let jkws_url = "https://www.googleapis.com/oauth2/v3/certs";
        let key_set = KeyStore::new_from(jkws_url).unwrap();

        token = token.replace("Bearer ", "");
        match key_set.verify(token.as_str()) {
            Ok(jwt) => {
                dbg!(jwt.payload());
                User::from_email(
                    jwt.payload().get_str("email").unwrap().to_string(),
                    connection,
                )
                .await
            }
            Err(e) => {
                dbg!(e.borrow());
                None
            }
        }
    }

    pub async fn set_mango_user(&mut self) -> Result<&mut User, Status> {
        let mango: Mangopay = Mangopay::init(
            env::var("MANGO_CLIENT_ID")
                .expect("MANGO_CLIENT_ID not set")
                .parse()
                .unwrap(),
            env::var("MANGO_API_KEY")
                .expect("MANGO_API_KEY not set")
                .parse()
                .unwrap(),
            "https://api.sandbox.mangopay.com/v2.01/".to_string(),
        );
        let user_infos = CreateUserBody {
            first_name: match self.user_name.split(' ').collect::<Vec<&str>>().get(0) {
                Some(val) => val.to_string(),
                None => "".to_string(),
            },
            last_name: match self.user_name.split(' ').collect::<Vec<&str>>().get(1) {
                Some(val) => val.to_string(),
                None => "".to_string(),
            },
            email: self.email.to_string(),
            user_category: "Payer".to_string(),
            tag: "Backend".to_string(),
            terms_and_conditions_accepted: true,
        };
        let user = match mango.create_user(&user_infos).await {
            Ok(user) => user,
            Err(e) => {
                dbg!(e);
                return Err(Status::FailedDependency);
            }
        };
        self.mango_pay_user_id = user.id;
        let wallet: Wallet = match mango
            .create_wallet(CreateWallet {
                owners: vec![self.mango_pay_user_id.to_string()],
                description: "User wallet".to_string(),
                currency: "EUR".to_string(),
                tag: "Backend Created".to_string(),
            })
            .await
        {
            Ok(wallet) => wallet,
            Err(e) => {
                dbg!(e);
                return Err(Status::FailedDependency);
            }
        };
        self.mango_pay_wallet_id = wallet.id;
        Ok(self)
    }

    pub async fn create_user(&mut self, connection: &Database) -> Result<Json<Value>, Status> {
        self.set_oauth_account().map_err(|_| Status::Unauthorized)?;
        self.set_mango_user().await?;
        match connection
            .collection::<User>("users")
            .insert_one(self.borrow(), None)
        {
            Ok(val) => {
                dbg!(val);
                Ok(Json(json!(self)))
            }
            Err(e) => {
                dbg!(e.borrow());
                Err(Status::AlreadyReported)
            }
        }
    }
}

impl Into<Bson> for User {
    fn into(self) -> bson::Bson {
        bson::to_bson(&self).unwrap()
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
                return Outcome::Forward(());
            }
        };
        match User::get_user_from_token(keys[0].to_string(), &db).await {
            Some(user) => Outcome::Success(user),
            None => Outcome::Forward(()),
        }
    }
}
