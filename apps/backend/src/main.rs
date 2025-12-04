use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/api/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Bonjour depuis Rust (Actix) !",
        "status": "Eco-friendly"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive(); // Pour le dev (à restreindre en prod)
        App::new()
            .wrap(cors)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
