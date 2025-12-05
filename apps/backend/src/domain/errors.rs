use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    OllamaTimeout,
    OllamaServiceUnavailable,
    InternalError,
    ValidationError,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::OllamaTimeout => write!(f, "Ollama service timed out"),
            AppError::OllamaServiceUnavailable => write!(f, "Ollama service is unavailable"),
            AppError::InternalError => write!(f, "Internal server error, please contact an administrator"),
            AppError::ValidationError => write!(f, "Validation error, please contact an administrator"),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::OllamaTimeout => HttpResponse::GatewayTimeout().json("Ollama service timed out"),
            AppError::OllamaServiceUnavailable => HttpResponse::ServiceUnavailable().json("Ollama service is unavailable"),
            AppError::InternalError => HttpResponse::InternalServerError().json("Internal server error, please contact an administrator"),
            AppError::ValidationError => HttpResponse::BadRequest().json("Validation error, please contact an administrator"),
        }
    }
}
