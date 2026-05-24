use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

pub fn success_response<T>(
    message: &str,
    data: Option<T>,
) -> ApiResponse<T> {
    ApiResponse {
        success: true,
        message: message.to_string(),
        data,
    }
}

pub fn error_response<T>(
    message: &str,
) -> ApiResponse<T> {
    ApiResponse {
        success: false,
        message: message.to_string(),
        data: None,
    }
}