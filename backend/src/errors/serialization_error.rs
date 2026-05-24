use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn json_serialization_failed(
) -> AppError {
    AppError::new(
        "JSON serialization failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn dto_conversion_failed(
) -> AppError {
    AppError::new(
        "DTO conversion failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn websocket_payload_serialization_failed(
) -> AppError {
    AppError::new(
        "WebSocket payload serialization failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}