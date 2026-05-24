use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn github_api_failed() -> AppError {
    AppError::new(
        "GitHub API request failed",
        StatusCode::BAD_GATEWAY,
    )
}

pub fn repository_not_found() -> AppError {
    AppError::new(
        "GitHub repository not found",
        StatusCode::NOT_FOUND,
    )
}

pub fn contribution_not_verified(
) -> AppError {
    AppError::new(
        "Contribution verification failed",
        StatusCode::BAD_REQUEST,
    )
}