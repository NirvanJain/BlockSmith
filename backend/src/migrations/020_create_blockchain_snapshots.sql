CREATE TABLE blockchain_snapshots (
    id BIGSERIAL PRIMARY KEY,
    snapshot_hash TEXT NOT NULL,
    total_blocks INTEGER NOT NULL,
    snapshot_data JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);