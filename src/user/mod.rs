pub mod model;

extern crate hmac;
extern crate sha2;

use std::borrow::Borrow;
use rocket::*;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use self::model::User;
use super::db;
use uuid::Uuid;
use std::str;

#[derive(Serialize, Deserialize)]
struct Credentials {
    email: String,
    token_id: String
}

#[post("/", format = "application/json", data = "<credentials>")]
fn create(credentials: Json<Credentials>) -> Result<Json<JsonValue>, Status> {
   let mut user = User {
       id: Uuid::new_v4().to_string(),
       email: credentials.email.to_string(),
       token_id: credentials.token_id.to_string()
 };
    let db = match db::connect() {
        Ok(db) => db,
        Err(e) => {
            dbg!(e);
            return Err(Status::FailedDependency)
        }
    };
    let res = user.create_or_get(db.borrow());
    res.map(|item| Json(item))
        .map_err(|_| Status::Conflict)
}

#[get("/<id>", format = "application/json")]
fn find(id: String) -> Result<Json<JsonValue>, Status> {
    let db = match db::connect() {
        Ok(db) => db,
        Err(e) => {
            dbg!(e);
            return Err(Status::FailedDependency)
        }
    };
    let res = User::infos(id, db.borrow());
    res.map(|item| Json(item))
        .map_err(|_| Status::NotFound)
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket
        .mount("/user", routes![create, find])
}

#[cfg(test)]
mod test {
    use mongodb::bson::doc;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};
    use crate::rocket;

    #[test]
    fn test_user_connection() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.post("/user/").header(ContentType::JSON).body(doc!
            {
                "email": "test@gmail.com",
                "token_id": "hashed"
            }.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
