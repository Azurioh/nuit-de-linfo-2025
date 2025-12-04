mod config;
mod api;
mod domain;
mod infrastructure;

use actix_cors::Cors;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::models::{Message, ConversationSession};
use infrastructure::ollama_client::OllamaClient;
use std::time::{Duration, Instant};
use actix_web::{rt, web, App, HttpServer};
use config::Config;
use api::chat_route::{chat_handler, hello};

type AppState = Arc<Mutex<HashMap<Uuid, ConversationSession>>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::init();

    let ollama_client = OllamaClient::new(config.ollama_url, config.model_name);
    let app_state: AppState = Arc::new(Mutex::new(HashMap::new()));

    // Background cleanup task
    let cleanup_state = app_state.clone();
    rt::spawn(async move {
        let mut interval = rt::time::interval(Duration::from_secs(60)); // Check every minute
        loop {
            interval.tick().await;
            let mut state = cleanup_state.lock().unwrap();
            let now = Instant::now();
            let initial_count = state.len();

            // Remove sessions inactive for more than 1 hour
            state.retain(|_, session| {
                now.duration_since(session.last_activity) < Duration::from_secs(3600)
            });

            if state.len() < initial_count {
                println!("Cleaned up {} inactive sessions", initial_count - state.len());
            }
        }
    });

    println!("Serveur lancé sur {}", config.server_address);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(ollama_client.clone()))
            .app_data(web::Data::new(app_state.clone()))
            .service(chat_handler)
            .service(hello)
    })
    .bind(Config::init().server_address)?
    .run()
    .await
}
