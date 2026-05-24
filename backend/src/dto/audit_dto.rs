use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Serialize,
    Deserialize,
)]
pub struct AuditLogDto {
    pub action_type: String,
    pub performed_by: String,
    pub entity_type: String,
    pub entity_id: String,
    pub timestamp: String,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
)]
pub struct AuditResponseDto {
    pub total_logs: usize,
    pub logs: Vec<AuditLogDto>,
}