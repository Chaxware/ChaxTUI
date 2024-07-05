use anyhow::anyhow;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/* ---------------------- Declarations ------------------------ */

// The struct that declares and receives the properties individual
// messages of the JSON response
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct MessageItem {
    pub id: String,
    pub text: String,
    pub channelId: String,
    pub userId: String,
    pub createdAt: String,
    pub updatedAt: String,
}

// The struct that receives the full JSON response for GET request
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct ApiResponse {
    pub messages: Vec<MessageItem>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct ChatGetResponse {
    pub id: String,
    pub name: String,
    pub hubId: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub messages: Vec<MessageItem>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct MessagePostResponse {
    pub success: bool,
    pub message: MessageItem,
}

pub struct Api {
    client: Client,
    chat_url: String,
}

#[derive(Debug)]
pub enum ApiError {
    Reqwest(anyhow::Error),
    SerdeJson(serde_json::Error),
}

/* ------------------- End of Declarations -------------------- */

/* ------------ Trait Implementations ----------- */

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        ApiError::Reqwest(anyhow::Error::new(error))
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        ApiError::SerdeJson(error)
    }
}

/* -------- End of Trait Implementations -------- */

impl Api {
    pub fn new(backend_base_url: String) -> Api {
        Api {
            client: Client::new(),
            chat_url: backend_base_url.clone(), // GET request url
        }
    }

    // Send GET request to get JSON of all messages in DB
    pub async fn fetch_messages(&self) -> Result<ChatGetResponse, anyhow::Error> {
        let response = self.client.get(&self.chat_url).send().await?;

        if response.status().is_success() {
            match response.json::<ChatGetResponse>().await {
                Ok(api_response) => Ok(api_response),
                Err(e) => Err(anyhow!(format!(
                    "Error when trying to parse messages: {}",
                    e
                ))),
            }
        } else {
            Err(anyhow!("Failed to fetch messages"))
        }
    }

    // Send POST request as JSON to add message to DB
    pub async fn send_message(&self, message_text: &String) -> Result<MessageItem, anyhow::Error> {
        let response = self
            .client
            .post(&self.chat_url)
            .json(&serde_json::json!({ "text": message_text, "userId": "Someone" }))
            .send()
            .await?;

        if response.status().is_success() {
            match response.json::<MessagePostResponse>().await {
                Ok(api_response) => Ok(api_response.message),
                Err(e) => Err(anyhow!(format!(
                    "Error when trying to parse message: {}",
                    e
                ))),
            }
        } else {
            Err(anyhow!("Failed to send message"))
        }
    }
}
