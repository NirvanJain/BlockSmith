CREATE INDEX idx_users_username
ON users(github_username);

CREATE INDEX idx_contributions_repository
ON contributions(repository);

CREATE INDEX idx_contributions_type
ON contributions(contribution_type);

CREATE INDEX idx_blocks_hash
ON blocks(hash);

CREATE INDEX idx_blocks_index
ON blocks(block_index);