CREATE TABLE contributions (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(id),
    repository TEXT NOT NULL,
    contribution_type TEXT NOT NULL,
    contribution_link TEXT NOT NULL,
    verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW()
);