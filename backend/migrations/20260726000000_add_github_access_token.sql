-- Add github_access_token column for storing the user's OAuth token
ALTER TABLE users ADD COLUMN IF NOT EXISTS github_access_token TEXT;
