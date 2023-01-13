use self::{model::Mangopay, card::{CardRegistrationResponse, CardRegistrationBody, UpdateCardRegistrationBody}};

pub mod card;
pub mod model;
pub mod user;
pub mod wallet;

impl Mangopay {
    pub async fn create_card_registration(
        self: &Mangopay,
        body: &CardRegistrationBody,
    ) -> Result<CardRegistrationResponse, reqwest::Error> {
        let user_response = match self
            .create_post_api_call("cardregistrations".parse().unwrap())
            .json(body)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(e),
        };
        let json_response = user_response.json().await?;
        return Ok(json_response);
    }

    pub async fn update_card_registration(
        self: &Mangopay,
        card_registration_id: String,
        body: &UpdateCardRegistrationBody,
    ) -> Result<CardRegistrationResponse, reqwest::Error> {
        let user_response = match self
            .create_put_api_call(format!("cardregistrations/{}", card_registration_id))
            .json(body)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(e),
        };
        let json_response = user_response.json().await?;
        return Ok(json_response);
    }

    pub async fn get_card_registration(
        self: &Mangopay,
        card_registration_id: String,
    ) -> Result<CardRegistrationResponse, reqwest::Error> {
        let user_response = match self
            .make_get_api_call(format!("cardregistrations/{}", card_registration_id))
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(e),
        };
        let json_response = user_response.json().await?;
        return Ok(json_response);
    }
}