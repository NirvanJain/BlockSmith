CREATE TABLE api_keys (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(id),
    api_key TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    last_used TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);