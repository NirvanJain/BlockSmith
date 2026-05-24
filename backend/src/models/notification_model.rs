use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct NotificationModel {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub message: String,
    pub is_read: bool,
    pub created_at: String,
}