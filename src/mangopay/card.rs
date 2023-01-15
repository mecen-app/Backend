use std::borrow::{Borrow, BorrowMut};

use rocket::serde::{json::Value, Deserialize, Serialize};

use super::model::Mangopay;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRegistrationBody {
    #[serde(rename = "Tag")]
    pub tag: String,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "CardType")]
    pub card_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRegistrationResponse {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Tag")]
    pub tag: Value,
    #[serde(rename = "CreationDate")]
    pub creation_date: i64,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "AccessKey")]
    pub access_key: String,
    #[serde(rename = "PreregistrationData")]
    pub preregistration_data: String,
    #[serde(rename = "RegistrationData")]
    pub registration_data: Value,
    #[serde(rename = "CardId")]
    pub card_id: Value,
    #[serde(rename = "CardType")]
    pub card_type: String,
    #[serde(rename = "CardRegistrationURL")]
    pub card_registration_url: String,
    #[serde(rename = "ResultCode")]
    pub result_code: Value,
    #[serde(rename = "ResultMessage")]
    pub result_message: Value,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Status")]
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCardRegistrationBody {
    #[serde(rename = "Tag")]
    pub tag: String,
    #[serde(rename = "RegistrationData")]
    pub registration_data: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardInfos {
    #[serde(rename = "ExpirationDate")]
    pub expiration_date: String,
    #[serde(rename = "Alias")]
    pub alias: String,
    #[serde(rename = "CardType")]
    pub card_type: String,
    #[serde(rename = "CardProvider")]
    pub card_provider: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Product")]
    pub product: String,
    #[serde(rename = "BankCode")]
    pub bank_code: String,
    #[serde(rename = "Active")]
    pub active: bool,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Validity")]
    pub validity: String,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Tag")]
    pub tag: Value,
    #[serde(rename = "CreationDate")]
    pub creation_date: i64,
    #[serde(rename = "Fingerprint")]
    pub fingerprint: String,
}
pub type ListCards = Vec<CardInfos>;

impl Mangopay {
    pub async fn list_cards_for_user(
        self: &Mangopay,
        user_id: String,
    ) -> Result<ListCards, reqwest::Error> {
        let cards_response = match self
            .make_get_api_call(format!("users/{}/cards", user_id))
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(e),
        };
        let json_response = cards_response.json().await?;
        return Ok(json_response);
    }
}
