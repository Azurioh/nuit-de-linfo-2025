use actix_web::{get, post, web, HttpResponse, Responder};
use futures::StreamExt;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use std::collections::HashMap;
use crate::domain::models::{ChatRequest, OllamaResponse, Message, ConversationSession};
use crate::infrastructure::ollama_client::OllamaClient;
use std::time::Instant;

type AppState = Arc<Mutex<HashMap<Uuid, ConversationSession>>>;

#[post("/chat")]
pub async fn chat_handler(
    req: web::Json<ChatRequest>,
    ollama: web::Data<OllamaClient>,
    state: web::Data<AppState>,
) -> impl Responder {
    let conversation_id = req.conversation_id.unwrap_or_else(Uuid::new_v4);

    let mut messages = {
        let mut state_guard = state.lock().unwrap();
        let session = state_guard.entry(conversation_id).or_insert_with(|| ConversationSession {
            messages: Vec::new(),
            last_activity: Instant::now(),
        });

        // Update last activity
        session.last_activity = Instant::now();

        session.messages.push(Message {
            role: "user".to_string(),
            content: req.prompt.clone(),
        });
        session.messages.clone()
    };

    match ollama.get_ref().clone().generate_chat_stream(messages).await {
        Ok(stream) => {
            let accumulated_response = Arc::new(Mutex::new(String::new()));
            let acc_clone = accumulated_response.clone();
            let state_clone = state.get_ref().clone();
            let conversation_id_clone = conversation_id;

            let response_stream = stream.map(move |result| {
                match result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        let mut chunk_text = String::new();
                        for line in text.lines() {
                            if !line.trim().is_empty() {
                                if let Ok(json) = serde_json::from_str::<OllamaResponse>(line) {
                                    chunk_text.push_str(&json.message.content);
                                }
                            }
                        }

                        if !chunk_text.is_empty() {
                            if let Ok(mut acc) = acc_clone.lock() {
                                acc.push_str(&chunk_text);
                            }
                        }

                        Ok(web::Bytes::from(chunk_text)) as Result<web::Bytes, actix_web::Error>
                    },
                    Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
                }
            });

            // Chain a final step to save the accumulated response
            let final_stream = response_stream.chain(futures::stream::once(async move {
                let content = {
                    let acc = accumulated_response.lock().unwrap();
                    acc.clone()
                };

                if !content.is_empty() {
                    let mut state_guard = state_clone.lock().unwrap();
                    if let Some(session) = state_guard.get_mut(&conversation_id_clone) {
                        session.messages.push(Message {
                            role: "assistant".to_string(),
                            content,
                        });
                        session.last_activity = Instant::now(); // Update activity on completion too
                    }
                }

                // Return an empty result to satisfy the stream type, but it won't yield data
                // Actually, chain expects the same Item type.
                // We return Ok(Bytes::new()) which is empty and harmless.
                Ok(web::Bytes::new())
            }));

            HttpResponse::Ok()
                .append_header(("X-Conversation-Id", conversation_id.to_string()))
                .streaming(final_stream)
        },
        Err(_) => HttpResponse::InternalServerError().body("Service indisponible"),
    }
}

#[get("/api/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"message": "Hello, world!"}))
}
