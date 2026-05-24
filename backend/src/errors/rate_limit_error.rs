use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn github_rate_limit_exceeded(
) -> AppError {
    AppError::new(
        "GitHub API rate limit exceeded",
        StatusCode::TOO_MANY_REQUESTS,
    )
}

pub fn too_many_requests() -> AppError {
    AppError::new(
        "Too many requests",
        StatusCode::TOO_MANY_REQUESTS,
    )
}