use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn repository_query_failed(
) -> AppError {
    AppError::new(
        "Repository query failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn repository_save_failed(
) -> AppError {
    AppError::new(
        "Repository save failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn repository_delete_failed(
) -> AppError {
    AppError::new(
        "Repository delete failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}