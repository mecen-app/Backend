
use serde_json::Value;

use super::model::Mangopay;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserBody {
    #[serde(rename = "FirstName")]
    pub first_name: String,
    #[serde(rename = "LastName")]
    pub last_name: String,
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "UserCategory")]
    pub user_category: String,
    #[serde(rename = "Tag")]
    pub tag: String,
    #[serde(rename = "TermsAndConditionsAccepted")]
    pub terms_and_conditions_accepted: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "Address")]
    pub address: Address,
    #[serde(rename = "FirstName")]
    pub first_name: String,
    #[serde(rename = "LastName")]
    pub last_name: String,
    #[serde(rename = "Birthday")]
    pub birthday: Value,
    #[serde(rename = "Nationality")]
    pub nationality: Value,
    #[serde(rename = "CountryOfResidence")]
    pub country_of_residence: Value,
    #[serde(rename = "Occupation")]
    pub occupation: Value,
    #[serde(rename = "IncomeRange")]
    pub income_range: Value,
    #[serde(rename = "ProofOfIdentity")]
    pub proof_of_identity: Value,
    #[serde(rename = "ProofOfAddress")]
    pub proof_of_address: Value,
    #[serde(rename = "Capacity")]
    pub capacity: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Tag")]
    pub tag: String,
    #[serde(rename = "CreationDate")]
    pub creation_date: i64,
    #[serde(rename = "PersonType")]
    pub person_type: String,
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "KYCLevel")]
    pub kyclevel: String,
    #[serde(rename = "TermsAndConditionsAccepted")]
    pub terms_and_conditions_accepted: bool,
    #[serde(rename = "TermsAndConditionsAcceptedDate")]
    pub terms_and_conditions_accepted_date: i64,
    #[serde(rename = "UserCategory")]
    pub user_category: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(rename = "AddressLine1")]
    pub address_line1: Value,
    #[serde(rename = "AddressLine2")]
    pub address_line2: Value,
    #[serde(rename = "City")]
    pub city: Value,
    #[serde(rename = "Region")]
    pub region: Value,
    #[serde(rename = "PostalCode")]
    pub postal_code: Value,
    #[serde(rename = "Country")]
    pub country: Value,
}

impl Mangopay {
    pub async fn create_user(
        self: &Mangopay,
        user_infos: &CreateUserBody,
    ) -> Result<User, reqwest::Error> {
        let user_response = match self
            .create_post_api_call("users/natural/".parse().unwrap())
            .json(user_infos)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(e),
        };
        let json_response = user_response.json().await?;
        return Ok(json_response);
    }

    pub async fn get_user(self: &Mangopay, user_id: String) -> Result<User, reqwest::Error> {
        let user_response = match self.make_get_api_call(format!("users/{}", user_id)).await {
            Ok(resp) => resp,
            Err(e) => return Err(e),
        };
        let json_response = user_response.json().await?;
        return Ok(json_response);
    }
}
