use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ChatRequest {
    pub prompt: String,
    pub conversation_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct OllamaChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
}

#[derive(Deserialize)]
pub struct OllamaResponse {
    pub message: Message,
    pub done: bool,
}

pub struct ConversationSession {
    pub messages: Vec<Message>,
    pub last_activity: std::time::Instant,
}

use lru::LruCache;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct AppStateInternal {
    pub sessions: HashMap<Uuid, ConversationSession>,
    pub cache: LruCache<String, String>,
}
