mod config;
mod api;
mod domain;
mod infrastructure;

use actix_cors::Cors;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::models::{Message, ConversationSession, AppStateInternal};
use infrastructure::ollama_client::OllamaClient;
use std::time::{Duration, Instant};
use actix_web::{rt, web, App, HttpServer};
use config::Config;
use api::chat_route::{chat_handler, hello};

use lru::LruCache;
use std::num::NonZeroUsize;
use actix_governor::{Governor, GovernorConfigBuilder};

type AppState = Arc<Mutex<AppStateInternal>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::init();

    let ollama_client = OllamaClient::new(config.ollama_url, config.model_name);
    let app_state: AppState = Arc::new(Mutex::new(AppStateInternal {
        sessions: HashMap::new(),
        cache: LruCache::new(NonZeroUsize::new(100).unwrap()),
    }));

    // Background cleanup task
    let cleanup_state = app_state.clone();
    rt::spawn(async move {
        let mut interval = rt::time::interval(Duration::from_secs(60)); // Check every minute
        loop {
            interval.tick().await;
            let mut state = cleanup_state.lock().unwrap();
            let now = Instant::now();
            let initial_count = state.sessions.len();

            // Remove sessions inactive for more than 1 hour
            state.sessions.retain(|_, session| {
                now.duration_since(session.last_activity) < Duration::from_secs(3600)
            });

            if state.sessions.len() < initial_count {
                println!("Cleaned up {} inactive sessions", initial_count - state.sessions.len());
            }
        }
    });

    println!("Serveur lancé sur {}", config.server_address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4321")
            .allowed_origin("https://nuit-de-linfo.azu-dev.fr")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE, actix_web::http::header::ACCEPT])
            .supports_credentials();

        let governor_conf = GovernorConfigBuilder::default()
            .period(Duration::from_secs(1))
            .burst_size(10)
            .finish()
            .unwrap();

        App::new()
            .wrap(cors)
            .wrap(Governor::new(&governor_conf))
            .app_data(web::Data::new(ollama_client.clone()))
            .app_data(web::Data::new(app_state.clone()))
            .service(chat_handler)
            .service(hello)
    })
    .bind(Config::init().server_address)?
    .run()
    .await
}
