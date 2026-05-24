use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn bad_request() -> AppError {
    AppError::new(
        "Bad API request",
        StatusCode::BAD_REQUEST,
    )
}

pub fn route_not_found() -> AppError {
    AppError::new(
        "API route not found",
        StatusCode::NOT_FOUND,
    )
}

pub fn method_not_allowed() -> AppError {
    AppError::new(
        "HTTP method not allowed",
        StatusCode::METHOD_NOT_ALLOWED,
    )
}

pub fn unsupported_media_type(
) -> AppError {
    AppError::new(
        "Unsupported media type",
        StatusCode::UNSUPPORTED_MEDIA_TYPE,
    )
}