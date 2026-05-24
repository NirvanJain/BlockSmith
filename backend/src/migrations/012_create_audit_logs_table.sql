CREATE TABLE audit_logs (
    id BIGSERIAL PRIMARY KEY,
    action_type TEXT NOT NULL,
    performed_by TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    metadata JSONB,
    created_at TIMESTAMP DEFAULT NOW()
);