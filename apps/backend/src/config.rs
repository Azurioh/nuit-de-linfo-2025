use std::env;

pub struct Config {
    pub server_address: String,
    pub mistral_api_key: String,
    pub model_name: String,
}

impl Config {
    pub fn init() -> self::Config {
        Config {
            server_address: env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
            mistral_api_key: env::var("MISTRAL_API_KEY").expect("MISTRAL_API_KEY must be set"),
            model_name: env::var("MODEL_NAME").unwrap_or_else(|_| "mistral-tiny".to_string()),
        }
    }
}
