use anyhow::anyhow;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/* ---------------------- Declarations ------------------------ */

// The struct that declares and receives the properties individual
// messages of the JSON response
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageItem {
    pub id: String,
    pub text: String,
    pub created_at: String,
}

// The struct that receives the full JSON response for GET request
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub messages: Vec<MessageItem>,
}

pub struct Api {
    client: Client,
    message_list_url: String,
    message_post_url: String,
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
            message_list_url: format!("{}/chat", backend_base_url), // GET request url
            message_post_url: format!("{}/chat/send", backend_base_url), // POST request url
        }
    }

    // Send GET request to get JSON of all messages in DB
    pub async fn fetch_messages(&self) -> Result<ApiResponse, anyhow::Error> {
        let response = self.client.get(&self.message_list_url).send().await?;
        if response.status().is_success() {
            match response.json::<ApiResponse>().await {
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
    pub async fn send_message(&self, message_text: &String) -> Result<(), anyhow::Error> {
        let response = self
            .client
            .post(&self.message_post_url)
            .json(&serde_json::json!({ "text": message_text }))
            .send()
            .await?;

        if !response.status().is_success() {
            Err(anyhow!("Failed to send message"))
        } else {
            Ok(())
        }
    }
}
