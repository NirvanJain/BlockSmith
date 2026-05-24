use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn invalid_chain() -> AppError {
    AppError::new(
        "Blockchain validation failed",
        StatusCode::BAD_REQUEST,
    )
}

pub fn block_creation_failed() -> AppError {
    AppError::new(
        "Failed to create blockchain block",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn invalid_hash() -> AppError {
    AppError::new(
        "Invalid block hash detected",
        StatusCode::BAD_REQUEST,
    )
}