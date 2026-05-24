use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppError {
    pub message: String,
    pub status: u16,
}

impl AppError {
    pub fn new(
        message: &str,
        status: StatusCode,
    ) -> Self {
        Self {
            message: message.to_string(),
            status: status.as_u16(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = serde_json::json!({
            "success": false,
            "error": self.message,
        });

        (
            StatusCode::from_u16(
                self.status,
            )
            .unwrap(),
            axum::Json(body),
        )
            .into_response()
    }
}