use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Serialize,
    Deserialize,
)]
pub struct ErrorDto {
    pub success: bool,
    pub error_code: String,
    pub message: String,
}

impl ErrorDto {
    pub fn new(
        error_code: &str,
        message: &str,
    ) -> Self {
        Self {
            success: false,
            error_code:
                error_code.to_string(),
            message:
                message.to_string(),
        }
    }
}