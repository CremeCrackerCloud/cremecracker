use actix_web::{http::StatusCode, ResponseError};
use paas_api::error::AppError;

#[test]
fn test_validation_error_response() {
    let error = AppError::ValidationError("Invalid input".to_string());
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn test_auth_error_response() {
    let error = AppError::AuthError("Unauthorized".to_string());
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn test_not_found_response() {
    let error = AppError::NotFound("Resource not found".to_string());
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_external_service_error_response() {
    let error = AppError::ExternalServiceError("External service failed".to_string());
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
}

#[test]
fn test_database_error_response() {
    let error = AppError::DatabaseError("Database error".to_string());
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_session_error_response() {
    let error = AppError::SessionError("Session error".to_string());
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
