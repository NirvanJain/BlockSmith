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
pub struct CacheMetadataModel {
    pub id: i64,
    pub cache_key: String,
    pub cache_type: String,
    pub created_at: String,
    pub expires_at: Option<String>,
}