use anyhow::anyhow;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageItem {
    pub id: String,
    pub text: String,
    pub created_at: String,
}

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

impl Api {
    pub fn new() -> Api {
        Api {
            client: Client::new(),
            message_list_url: "http://localhost:8787/chat".into(),
            message_post_url: "http://localhost:8787/chat/send".into(),
        }
    }

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
