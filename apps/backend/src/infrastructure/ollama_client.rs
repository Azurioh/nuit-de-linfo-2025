use std::sync::Arc;
use reqwest::Client;
use crate::domain::models::OllamaRequest;

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
            base_url: Arc::new(url),
            model: Arc::new(model),
        }
    }


    pub async fn generate_completion_stream(self, prompt: String) -> Result<impl futures::Stream<Item = Result<actix_web::web::Bytes, reqwest::Error>>, reqwest::Error> {
        let request_body = OllamaRequest {
            model: self.model.as_ref().clone(),
            prompt,
            stream: true,
        };

        let res = self.client
            .post(self.base_url.as_ref())
            .json(&request_body)
            .send()
            .await?;

        Ok(res.bytes_stream())
    }
}
