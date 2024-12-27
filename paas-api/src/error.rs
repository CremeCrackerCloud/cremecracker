use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use serde_json::json;
use actix_session::{SessionGetError, SessionInsertError};

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "Database error: {}", _0)]
    DatabaseError(String),

    #[display(fmt = "Authentication error: {}", _0)]
    AuthError(String),

    #[display(fmt = "Validation error: {}", _0)]
    #[allow(dead_code)]
    ValidationError(String),

    #[display(fmt = "External service error: {}", _0)]
    ExternalServiceError(String),

    #[display(fmt = "Session error: {}", _0)]
    SessionError(String),

    #[display(fmt = "Not found: {}", _0)]
    #[allow(dead_code)]
    NotFound(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::AuthError(msg) => HttpResponse::Unauthorized().json(json!({
                "error": msg
            })),
            AppError::ValidationError(msg) => HttpResponse::BadRequest().json(json!({
                "error": msg
            })),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(json!({
                "error": msg
            })),
            AppError::ExternalServiceError(msg) => HttpResponse::BadGateway().json(json!({
                "error": msg
            })),
            _ => HttpResponse::InternalServerError().json(json!({
                "error": self.to_string()
            })),
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::AuthError(_) => StatusCode::UNAUTHORIZED,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::ExternalServiceError(_) => StatusCode::BAD_GATEWAY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<SessionGetError> for AppError {
    fn from(err: SessionGetError) -> Self {
        AppError::SessionError(err.to_string())
    }
}

impl From<SessionInsertError> for AppError {
    fn from(err: SessionInsertError) -> Self {
        AppError::SessionError(err.to_string())
    }
}
