CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    github_username TEXT NOT NULL UNIQUE,
    avatar_url TEXT,
    reputation_score INTEGER DEFAULT 0,
    total_contributions INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW()
);