use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    OllamaTimeout,
    OllamaServiceUnavailable,
    InternalError(String),
    ValidationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::OllamaTimeout => write!(f, "Ollama service timed out"),
            AppError::OllamaServiceUnavailable => write!(f, "Ollama service is unavailable"),
            AppError::InternalError(msg) => write!(f, "Internal server error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::OllamaTimeout => HttpResponse::GatewayTimeout().json("Ollama service timed out"),
            AppError::OllamaServiceUnavailable => HttpResponse::ServiceUnavailable().json("Ollama service is unavailable"),
            AppError::InternalError(msg) => HttpResponse::InternalServerError().json(format!("Internal server error: {}", msg)),
            AppError::ValidationError(msg) => HttpResponse::BadRequest().json(format!("Validation error: {}", msg)),
        }
    }
}
