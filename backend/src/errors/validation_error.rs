use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn invalid_input() -> AppError {
    AppError::new(
        "Invalid input provided",
        StatusCode::BAD_REQUEST,
    )
}

pub fn missing_fields() -> AppError {
    AppError::new(
        "Required fields missing",
        StatusCode::BAD_REQUEST,
    )
}

pub fn invalid_repository_format(
) -> AppError {
    AppError::new(
        "Repository format invalid",
        StatusCode::BAD_REQUEST,
    )
}