CREATE TABLE blocks (
    id BIGSERIAL PRIMARY KEY,
    block_index INTEGER NOT NULL,
    contributor TEXT NOT NULL,
    repository TEXT NOT NULL,
    contribution_type TEXT NOT NULL,
    contribution_link TEXT NOT NULL,
    previous_hash TEXT NOT NULL,
    hash TEXT NOT NULL,
    timestamp TEXT NOT NULL
);