mod config;
mod api;
mod domain;
mod infrastructure;

use actix_web::{web, App, HttpServer};
use config::Config;
use infrastructure::ollama_client::OllamaClient;
use api::chat_route::{chat_handler, hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::init();

    let ollama_client = OllamaClient::new(config.ollama_url, config.model_name);

    println!("Serveur lancé sur {}", config.server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ollama_client.clone()))
            .service(chat_handler)
            .service(hello)
    })
    .bind(Config::init().server_address)?
    .run()
    .await
}
