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
pub struct SessionModel {
    pub id: i64,
    pub user_id: i64,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: String,
    pub created_at: String,
}