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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MistralChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MistralResponse {
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub delta: Delta,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Delta {
    pub content: Option<String>,
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
    pub redis: redis::Client,
}
