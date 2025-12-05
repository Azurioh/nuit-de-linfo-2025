use std::sync::Arc;
use reqwest::Client;
use crate::domain::models::{OllamaChatRequest, Message};
use crate::domain::errors::AppError;

#[derive(Clone)]
pub struct OllamaClient {
    client: Client,
    base_url: Arc<String>,
    model: Arc<String>,
}

impl OllamaClient {
    pub fn new(url: String, model: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_else(|_| Client::new()),
            base_url: Arc::new(url.replace("/api/generate", "/api/chat")), // Ensure we use the correct base path if user provided full URL
            model: Arc::new(model),
        }
    }


    pub async fn generate_chat_stream(self, messages: Vec<Message>) -> Result<impl futures::Stream<Item = Result<actix_web::web::Bytes, reqwest::Error>>, AppError> {
        let request_body = OllamaChatRequest {
            model: self.model.as_ref().clone(),
            messages,
            stream: true,
        };

        let res = self.client
            .post(self.base_url.as_ref())
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    AppError::OllamaTimeout
                } else if e.is_connect() {
                    AppError::OllamaServiceUnavailable
                } else {
                    AppError::InternalError(e.to_string())
                }
            })?;

        Ok(res.bytes_stream())
    }

    pub async fn generate_chat_completion(&self, messages: Vec<Message>) -> Result<crate::domain::models::OllamaResponse, AppError> {
        let request_body = OllamaChatRequest {
            model: self.model.as_ref().clone(),
            messages,
            stream: false,
        };

        let res = self.client
            .post(self.base_url.as_ref())
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    AppError::OllamaTimeout
                } else if e.is_connect() {
                    AppError::OllamaServiceUnavailable
                } else {
                    AppError::InternalError(e.to_string())
                }
            })?;

        res.json::<crate::domain::models::OllamaResponse>()
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))
    }
}
