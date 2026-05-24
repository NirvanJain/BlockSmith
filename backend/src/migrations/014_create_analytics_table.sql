CREATE TABLE analytics (
    id BIGSERIAL PRIMARY KEY,
    metric_name TEXT NOT NULL,
    metric_value BIGINT DEFAULT 0,
    recorded_at TIMESTAMP DEFAULT NOW()
);