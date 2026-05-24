INSERT INTO users
(
    github_username,
    avatar_url,
    reputation_score,
    total_contributions
)
VALUES
(
    'nirvanjain',
    'https://github.com/github.png',
    120,
    15
);

INSERT INTO contributions
(
    user_id,
    repository,
    contribution_type,
    contribution_link,
    verified
)
VALUES
(
    1,
    'BlockSmith',
    'pull_request',
    'https://github.com/example/pull/1',
    TRUE
);

INSERT INTO blocks
(
    block_index,
    contributor,
    repository,
    contribution_type,
    contribution_link,
    previous_hash,
    hash,
    timestamp
)
VALUES
(
    1,
    'nirvanjain',
    'BlockSmith',
    'pull_request',
    'https://github.com/example/pull/1',
    '000000000000',
    'abc123def456',
    NOW()::TEXT
);