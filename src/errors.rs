use actix_web::{HttpResponse, http::StatusCode, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Error saat scraping: {0}")]
    ScrapingError(String),
    
    #[error("Error saat request HTTP: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("Error saat mengolah JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Error internal: {0}")]
    InternalError(String),
    
    #[error("Tidak ditemukan: {0}")]
    NotFoundError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    status: &'static str,
    message: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::ScrapingError(_) => StatusCode::BAD_GATEWAY,
            AppError::HttpError(_) => StatusCode::BAD_GATEWAY,
            AppError::JsonError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFoundError(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let error_response = ErrorResponse {
            status: "error",
            message: self.to_string(),
        };
        
        HttpResponse::build(status).json(error_response)
    }
} 