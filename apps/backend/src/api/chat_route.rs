use actix_web::{get, post, web, HttpResponse, Responder};
use futures::StreamExt;
use crate::domain::models::{ChatRequest, OllamaResponse};
use crate::infrastructure::ollama_client::OllamaClient;

#[post("/chat")]
pub async fn chat_handler(
    req: web::Json<ChatRequest>,
    ollama: web::Data<OllamaClient>
) -> impl Responder {
    match ollama.get_ref().clone().generate_completion_stream(req.prompt.clone()).await {
        Ok(stream) => {
            let response_stream = stream.map(|result| {
                match result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        let mut response_text = String::new();
                        for line in text.lines() {
                            if !line.trim().is_empty() {
                                if let Ok(json) = serde_json::from_str::<OllamaResponse>(line) {
                                    response_text.push_str(&json.response);
                                }
                            }
                        }
                        Ok(web::Bytes::from(response_text)) as Result<web::Bytes, actix_web::Error>
                    },
                    Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
                }
            });
            HttpResponse::Ok().streaming(response_stream)
        },
        Err(_) => HttpResponse::InternalServerError().body("Service indisponible"),
    }
}

#[get("/api/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"message": "Hello, world!"}))
}
