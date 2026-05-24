use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn database_connection_failed(
) -> AppError {
    AppError::new(
        "Database connection failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn query_execution_failed(
) -> AppError {
    AppError::new(
        "Database query failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn record_not_found() -> AppError {
    AppError::new(
        "Record not found",
        StatusCode::NOT_FOUND,
    )
}