use std::env;

pub struct Config {
    pub server_address: String,
    pub mistral_api_key: String,
    pub model_name: String,
    pub redis_url: String,
}

impl Config {
    pub fn init() -> self::Config {
        Config {
            server_address: env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set"),
            mistral_api_key: env::var("MISTRAL_API_KEY").expect("MISTRAL_API_KEY must be set"),
            model_name: env::var("MODEL_NAME").expect("MODEL_NAME must be set"),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
        }
    }
}
