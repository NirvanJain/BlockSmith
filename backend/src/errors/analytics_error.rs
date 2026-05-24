use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn analytics_generation_failed(
) -> AppError {
    AppError::new(
        "Analytics generation failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn leaderboard_generation_failed(
) -> AppError {
    AppError::new(
        "Leaderboard generation failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn metrics_collection_failed(
) -> AppError {
    AppError::new(
        "Metrics collection failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}