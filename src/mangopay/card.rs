use rocket::serde::{json::Value, Deserialize, Serialize};

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
