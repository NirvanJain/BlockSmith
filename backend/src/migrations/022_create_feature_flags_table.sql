CREATE TABLE feature_flags (
    id BIGSERIAL PRIMARY KEY,
    feature_name TEXT UNIQUE NOT NULL,
    enabled BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW()
);