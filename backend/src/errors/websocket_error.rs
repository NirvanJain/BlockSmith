use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn websocket_connection_failed(
) -> AppError {
    AppError::new(
        "WebSocket connection failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn websocket_broadcast_failed(
) -> AppError {
    AppError::new(
        "Realtime event broadcast failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn invalid_websocket_payload(
) -> AppError {
    AppError::new(
        "Invalid websocket payload",
        StatusCode::BAD_REQUEST,
    )
}