use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn queue_job_failed() -> AppError {
    AppError::new(
        "Queue job processing failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn worker_failed() -> AppError {
    AppError::new(
        "Background worker failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn queue_connection_failed(
) -> AppError {
    AppError::new(
        "Queue connection failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}