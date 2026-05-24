use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Serialize,
    Deserialize,
)]
pub struct NotificationDto {
    pub title: String,
    pub message: String,
    pub notification_type: String,
    pub timestamp: String,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
)]
pub struct NotificationResponseDto {
    pub success: bool,
    pub notifications:
        Vec<NotificationDto>,
}