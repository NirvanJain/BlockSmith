CREATE TABLE verification_queue (
    id BIGSERIAL PRIMARY KEY,
    contribution_id BIGINT REFERENCES contributions(id),
    status TEXT DEFAULT 'pending',
    retry_count INTEGER DEFAULT 0,
    queued_at TIMESTAMP DEFAULT NOW(),
    processed_at TIMESTAMP
);