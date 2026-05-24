use axum::http::StatusCode;

use crate::errors::app_error::AppError;

pub fn scheduler_job_failed(
) -> AppError {
    AppError::new(
        "Scheduled job execution failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn cron_execution_failed(
) -> AppError {
    AppError::new(
        "Cron execution failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn cleanup_job_failed(
) -> AppError {
    AppError::new(
        "Cleanup job failed",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}