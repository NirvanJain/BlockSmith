CREATE TABLE webhook_logs (
    id BIGSERIAL PRIMARY KEY,
    event_type TEXT NOT NULL,
    repository TEXT NOT NULL,
    contributor TEXT NOT NULL,
    payload JSONB NOT NULL,
    received_at TIMESTAMP DEFAULT NOW()
);