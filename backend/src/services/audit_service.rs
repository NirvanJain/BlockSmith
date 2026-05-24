use crate::models::{
    audit_model::AuditModel,
};

pub struct AuditService;

impl AuditService {
    pub fn log_action(
        action_type: String,
        performed_by: String,
        entity_type: String,
        entity_id: String,
    ) -> AuditModel {
        AuditModel {
            id: 0,
            action_type,
            performed_by,
            entity_type,
            entity_id,
            metadata: None,
            created_at:
                chrono::Utc::now()
                    .to_rfc3339(),
        }
    }
}