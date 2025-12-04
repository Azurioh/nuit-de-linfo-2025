use std::env;

pub struct Config {
    pub server_address: String,
    pub ollama_url: String,
    pub model_name: String,
}

impl Config {
    pub fn init() -> self::Config {
        Config {
            server_address: env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
            ollama_url: env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434/api/generate".to_string()),
            model_name: env::var("MODEL_NAME").unwrap_or_else(|_| "ministral-3:3b".to_string()),
        }
    }
}
