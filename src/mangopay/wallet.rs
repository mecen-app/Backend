

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use super::model::Mangopay;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWallet {
    #[serde(rename = "Owners")]
    pub owners: Vec<String>,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Tag")]
    pub tag: String,
}

pub type ListWallets = Vec<Wallet>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Owners")]
    pub owners: Vec<String>,
    #[serde(rename = "Balance")]
    pub balance: Balance,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "FundsType")]
    pub funds_type: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Tag")]
    pub tag: Value,
    #[serde(rename = "CreationDate")]
    pub creation_date: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Amount")]
    pub amount: i64,
}

impl Mangopay {
    pub async fn create_wallet(
        self: &Mangopay,
        wallet: CreateWallet,
    ) -> Result<Wallet, reqwest::Error> {
        let wallet_response = match self
            .create_post_api_call("wallets/".to_string())
            .json(&wallet)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(e),
        };
        let json_response = wallet_response.json().await?;
        return Ok(json_response);
    }

    pub async fn list_wallets(
        self: &Mangopay,
        user_id: String,
    ) -> Result<ListWallets, reqwest::Error> {
        let wallet_response = match self
            .make_get_api_call(format!("users/{}/wallets", user_id))
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(e),
        };
        let json_response = wallet_response.json().await?;
        return Ok(json_response);
    }
}
