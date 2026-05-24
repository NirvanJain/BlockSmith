CREATE TABLE leaderboard (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(id),
    reputation_score INTEGER DEFAULT 0,
    verified_contributions INTEGER DEFAULT 0,
    rank_position INTEGER,
    updated_at TIMESTAMP DEFAULT NOW()
);