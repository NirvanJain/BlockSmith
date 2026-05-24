CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    github_username TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE contributions (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(id),
    repo_name TEXT NOT NULL,
    contribution_type TEXT NOT NULL,
    contribution_link TEXT NOT NULL,
    verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE blocks (
    id BIGSERIAL PRIMARY KEY,
    block_index INTEGER NOT NULL,
    contribution_id BIGINT REFERENCES contributions(id),
    previous_hash TEXT NOT NULL,
    hash TEXT NOT NULL,
    timestamp TEXT NOT NULL
);