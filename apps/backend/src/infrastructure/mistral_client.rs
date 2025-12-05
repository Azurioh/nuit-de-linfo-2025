use std::sync::Arc;
use reqwest::Client;
use crate::domain::models::{MistralChatRequest, Message};
use crate::domain::errors::AppError;

#[derive(Clone)]
pub struct MistralClient {
    client: Client,
    api_key: Arc<String>,
    model: Arc<String>,
}

impl MistralClient {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_else(|_| Client::new()),
            api_key: Arc::new(api_key),
            model: Arc::new(model),
        }
    }

    pub async fn generate_chat_stream(&self, messages: Vec<Message>) -> Result<impl futures::Stream<Item = Result<actix_web::web::Bytes, reqwest::Error>>, AppError> {
        let request_body = MistralChatRequest {
            model: self.model.as_ref().clone(),
            messages,
            stream: true,
        };

        let res = self.client
            .post("https://api.mistral.ai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    AppError::OllamaTimeout // We can reuse this error or rename it
                } else if e.is_connect() {
                    AppError::OllamaServiceUnavailable
                } else {
                    AppError::InternalError(e.to_string())
                }
            })?;

        if !res.status().is_success() {
             let error_text = res.text().await.unwrap_or_default();
             return Err(AppError::InternalError(format!("Mistral API Error: {}", error_text)));
        }

        Ok(res.bytes_stream())
    }
}
