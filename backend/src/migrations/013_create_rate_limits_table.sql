CREATE TABLE rate_limits (
    id BIGSERIAL PRIMARY KEY,
    identifier TEXT NOT NULL,
    request_count INTEGER DEFAULT 0,
    window_start TIMESTAMP DEFAULT NOW(),
    created_at TIMESTAMP DEFAULT NOW()
);