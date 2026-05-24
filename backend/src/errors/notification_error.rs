use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn notification_send_failed(
) -> AppError {
    AppError::new(
        "Failed to send notification",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn websocket_notification_failed(
) -> AppError {
    AppError::new(
        "Realtime notification failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn invalid_notification_payload(
) -> AppError {
    AppError::new(
        "Invalid notification payload",
        StatusCode::BAD_REQUEST,
    )
}