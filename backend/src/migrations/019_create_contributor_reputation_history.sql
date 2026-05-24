CREATE TABLE contributor_reputation_history (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(id),
    reputation_score INTEGER NOT NULL,
    recorded_at TIMESTAMP DEFAULT NOW()
);