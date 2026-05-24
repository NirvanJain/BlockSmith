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
pub struct AuditModel {
    pub id: i64,
    pub action_type: String,
    pub performed_by: String,
    pub entity_type: String,
    pub entity_id: String,
    pub metadata:
        Option<serde_json::Value>,
    pub created_at: String,
}