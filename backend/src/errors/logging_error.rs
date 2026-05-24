use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn log_write_failed() -> AppError {
    AppError::new(
        "Failed to write logs",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn audit_log_failed() -> AppError {
    AppError::new(
        "Audit logging failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn logger_initialization_failed(
) -> AppError {
    AppError::new(
        "Logger initialization failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}