-- Make clerk_user_id optional (deprecated, kept for backwards compat)
ALTER TABLE users ALTER COLUMN clerk_user_id DROP NOT NULL;

-- Ensure github_id has a unique constraint (used for upsert)
-- The original migration already has UNIQUE on github_username but not github_id
ALTER TABLE users ADD CONSTRAINT users_github_id_unique UNIQUE (github_id);
