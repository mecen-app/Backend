use reqwest::{Client, RequestBuilder, Response};
use serde_json::Value;

pub struct Mangopay {
    client_id: String,
    api_key: String,
    authorization_token: String,
    mango_api_url_with_user_id: String,
}

pub type GetCardsResponse = Vec<CardResponse>;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardResponse {
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

impl Mangopay {
    pub fn init(client_id: String, api_key: String, mango_url: String) -> Self {
        let formatted_token = format!("{}:{}", client_id, api_key);
        let authorization_token = base64::encode(formatted_token);
        let mango_api_url_with_user_id: String = format!("{}{}", mango_url, client_id);
        let mango_infos: Mangopay = Mangopay {
            client_id,
            api_key,
            authorization_token,
            mango_api_url_with_user_id,
        };
        mango_infos
    }

    pub async fn make_get_api_call(self: &Mangopay, api_url: String) -> reqwest::Result<Response> {
        let client: Client = reqwest::Client::new();
        client
            .get(format!("{}/{}", self.mango_api_url_with_user_id, api_url))
            .header(
                "Authorization",
                format!("Basic {}", self.authorization_token),
            )
            .send()
            .await
    }

    pub fn create_put_api_call(self: &Mangopay, api_url: String) -> RequestBuilder {
        let client: Client = reqwest::Client::new();
        client
            .put(format!("{}/{}", self.mango_api_url_with_user_id, api_url))
            .header(
                "Authorization",
                format!("Basic {}", self.authorization_token),
            )
    }

    pub fn create_post_api_call(self: &Mangopay, api_url: String) -> RequestBuilder {
        let client: Client = reqwest::Client::new();
        client
            .post(format!("{}/{}", self.mango_api_url_with_user_id, api_url))
            .header(
                "Authorization",
                format!("Basic {}", self.authorization_token),
            )
    }
}
