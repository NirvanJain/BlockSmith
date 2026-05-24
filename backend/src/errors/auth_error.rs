use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn unauthorized() -> AppError {
    AppError::new(
        "Unauthorized access",
        StatusCode::UNAUTHORIZED,
    )
}

pub fn invalid_token() -> AppError {
    AppError::new(
        "Invalid JWT token",
        StatusCode::UNAUTHORIZED,
    )
}

pub fn oauth_failed() -> AppError {
    AppError::new(
        "GitHub OAuth failed",
        StatusCode::BAD_REQUEST,
    )
}