use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn contribution_processing_failed(
) -> AppError {
    AppError::new(
        "Contribution processing failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn verification_pipeline_failed(
) -> AppError {
    AppError::new(
        "Contribution verification pipeline failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn reputation_calculation_failed(
) -> AppError {
    AppError::new(
        "Reputation calculation failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}