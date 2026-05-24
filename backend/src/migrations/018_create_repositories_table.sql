CREATE TABLE repositories (
    id BIGSERIAL PRIMARY KEY,
    github_repo_id BIGINT UNIQUE,
    owner TEXT NOT NULL,
    name TEXT NOT NULL,
    full_name TEXT NOT NULL UNIQUE,
    description TEXT,
    stars INTEGER DEFAULT 0,
    forks INTEGER DEFAULT 0,
    language TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);