use crate::mangopay::card::{CardRegistrationBody, UpdateCardRegistrationBody};
use crate::mangopay::model::Mangopay;
use crate::user::model::User;
use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::{get, post, routes, Build, Rocket};
use std::env;

#[post("/init_registration")]
pub async fn init_card_registration(user: User) -> Result<Json<Value>, Status> {
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
    let card_registration_result = match mango
        .create_card_registration(&CardRegistrationBody {
            tag: "Tag".to_string(),
            user_id: user.mango_pay_user_id.to_owned(),
            currency: "EUR".to_string(),
            card_type: "CB_VISA_MASTERCARD".to_string(),
        })
        .await
    {
        Ok(res) => res,
        Err(e) => {
            dbg!(e);
            return Err(Status::FailedDependency);
        }
    };
    Ok(Json::from(json!(card_registration_result)))
}

#[post("/finish_card_registration?<registration_id>", data = "<input>")]
pub async fn finish_card_registration(
    _user: User,
    input: Json<UpdateCardRegistrationBody>,
    registration_id: String,
) -> Result<Json<Value>, Status> {
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
    let result = mango
        .update_card_registration(registration_id, &input)
        .await
        .unwrap();
    return Ok(Json::from(json!(result)));
}

#[get("/list")]
pub async fn get_user_cards(_user: User) -> Result<Json<Value>, Status> {
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
    let result = match mango.list_cards_for_user(_user.mango_pay_user_id).await {
        Ok(v) => v,
        Err(e) => {
            dbg!(e);
            return Err(Status::BadRequest);
        }
    };
    return Ok(Json::from(json!(result)));
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/card",
        routes![
            init_card_registration,
            finish_card_registration,
            get_user_cards
        ],
    )
}
