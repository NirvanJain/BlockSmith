use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn cache_miss() -> AppError {
    AppError::new(
        "Cache entry not found",
        StatusCode::NOT_FOUND,
    )
}

pub fn cache_write_failed(
) -> AppError {
    AppError::new(
        "Failed to write cache",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn redis_connection_failed(
) -> AppError {
    AppError::new(
        "Redis connection failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}