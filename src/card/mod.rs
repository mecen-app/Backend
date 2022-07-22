use mangopay::card::{CardRegistrationBody, UpdateCardRegistrationBody};
use mangopay::Mangopay;
use rocket::http::Status;
use rocket::{Build, Rocket, routes, post};
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use crate::user::model::User;

#[post("/init_registration")]
pub async fn init_card_registration(user: User) -> Result<Json<Value>, Status> {
    let mango: Mangopay = Mangopay::init(
        env!("MANGO_CLIENT_ID").parse().unwrap(),
        env!("MANGO_API_KEY").parse().unwrap(),
        "https://api.sandbox.mangopay.com/v2.01/".to_string()
    );
    let card_registration_result = match mango.create_card_registration(&CardRegistrationBody{
        tag: "Tag".to_string(),
        user_id: user.mango_pay_user_id.to_owned(),
        currency: "EUR".to_string(),
        card_type: "CB_VISA_MASTERCARD".to_string()
    }).await {
        Ok(res) => res,
        Err(e) => {
            dbg!(e);
            return Err(Status::FailedDependency)
        }
    };
    Ok(Json::from(json!(card_registration_result)))
}

#[post("/finish_card_registration?<registration_id>", data = "<input>")]
pub async fn finish_card_registration(user: User, input: Json<UpdateCardRegistrationBody>, registration_id: String) -> Result<Json<Value>, Status> {
    let mango: Mangopay = Mangopay::init(
        env!("MANGO_CLIENT_ID").parse().unwrap(),
        env!("MANGO_API_KEY").parse().unwrap(),
        "https://api.sandbox.mangopay.com/v2.01/".to_string()
    );
    let result = mango.update_card_registration(registration_id, &input).await.unwrap();
    return Ok(Json::from(json!(result)));
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/card", routes![init_card_registration])
}
