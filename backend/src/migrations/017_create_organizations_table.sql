CREATE TABLE organizations (
    id BIGSERIAL PRIMARY KEY,
    github_org_id BIGINT UNIQUE,
    login TEXT NOT NULL UNIQUE,
    avatar_url TEXT,
    description TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);