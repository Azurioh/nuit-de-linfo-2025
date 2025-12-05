mod config;
mod api;
mod domain;
mod infrastructure;

use actix_cors::Cors;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::models::{Message, ConversationSession, AppStateInternal};
use redis::Client as RedisClient;
use infrastructure::mistral_client::MistralClient;
use std::time::{Duration, Instant};
use actix_web::{rt, web, App, HttpServer, middleware::Compress};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;
use config::Config;
use api::chat_route::{chat_handler, hello};

use lru::LruCache;
use std::num::NonZeroUsize;
use actix_governor::{Governor, GovernorConfigBuilder};

type AppState = Arc<Mutex<AppStateInternal>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::init();

    let mistral_client = MistralClient::new(config.mistral_api_key, config.model_name);
    let redis_client = RedisClient::open("redis://redis:6379/").expect("Failed to create Redis client");
    let app_state: AppState = Arc::new(Mutex::new(AppStateInternal {
        sessions: HashMap::new(),
        cache: LruCache::new(NonZeroUsize::new(100).unwrap()),
        redis: redis_client,
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

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4321")
            .allowed_origin("http://127.0.0.1:4321")
            .allowed_origin("https://nuit-de-linfo.azu-dev.fr")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE, actix_web::http::header::ACCEPT])
            .expose_headers(vec!["X-Conversation-Id"])
            .max_age(3600)
            .supports_credentials();

        // HTTP/2 TLS Config (Self-signed for dev)
        // Note: In a real scenario, we would load certs from files.
        // For this hackathon, we'll assume certs are generated or just use HTTP/1.1 if certs missing but user asked for HTTP/2.
        // Actually, let's keep it simple and just bind port 8080 without TLS for now as generating certs in docker is complex.
        // User asked for HTTP/2 which REQUIRES TLS.
        // I will add a comment that for HTTP/2 we need TLS, but for now we stick to HTTP/1.1 to ensure it works in docker without cert volume.
        // WAIT, I can generate a self-signed cert in main.rs using rcgen! No, that adds dependency.
        // I will stick to HTTP/1.1 but with optimizations.
        // User explicitly asked for HTTP/2.
        // I will try to load certs if present, else fallback.

        let governor_conf = GovernorConfigBuilder::default()
            .period(Duration::from_secs(1))
            .burst_size(10)
            .finish()
            .unwrap();

        App::new()
            .wrap(Governor::new(&governor_conf)) // Rate limiting first
            .wrap(cors) // CORS last (so it executes first and handles OPTIONS)
            .app_data(web::Data::new(mistral_client.clone()))
            .app_data(web::Data::new(app_state.clone()))
            .service(chat_handler)
            .service(
                web::scope("/api")
                    .wrap(Compress::default())
                    .service(hello)
            )
    });



    // Try to load TLS config for HTTP/2
    let tls_config = {
        // Only try to load if files exist
        if std::path::Path::new("cert.pem").exists() && std::path::Path::new("key.pem").exists() {
            let cert_file = &mut BufReader::new(File::open("cert.pem")?);
            let key_file = &mut BufReader::new(File::open("key.pem")?);
            let cert_chain = certs(cert_file).unwrap().into_iter().map(Certificate).collect();
            let mut keys = pkcs8_private_keys(key_file).unwrap().into_iter().map(PrivateKey).collect::<Vec<_>>();

            if keys.is_empty() {
                 None
            } else {
                let config = ServerConfig::builder()
                    .with_safe_defaults()
                    .with_no_client_auth()
                    .with_single_cert(cert_chain, keys.remove(0))
                    .expect("bad certificate/key");
                Some(config)
            }
        } else {
            None
        }
    };

    if let Some(config) = tls_config {
        println!("HTTP/2 Enabled (TLS) running on {}", Config::init().server_address);
        server.bind_rustls(Config::init().server_address, config)?.run().await
    } else {
        println!("HTTP/1.1 Enabled (No TLS found) running on {}", Config::init().server_address);
        server.bind(Config::init().server_address)?.run().await
    }
}
