-- BlockSmith D1 Schema (SQLite)
-- Adapted from PostgreSQL migration for Cloudflare D1

-- USERS TABLE
CREATE TABLE IF NOT EXISTS users (
  id TEXT PRIMARY KEY,
  github_id TEXT UNIQUE,
  github_username TEXT UNIQUE,
  github_access_token TEXT,
  email TEXT UNIQUE,
  name TEXT,
  avatar_url TEXT,
  reputation_score INTEGER DEFAULT 0,
  trust_score INTEGER DEFAULT 0,
  total_contributions INTEGER DEFAULT 0,
  xp INTEGER DEFAULT 0,
  level INTEGER DEFAULT 1,
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT DEFAULT (datetime('now'))
);

-- PROFILES TABLE
CREATE TABLE IF NOT EXISTS profiles (
  user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
  bio TEXT,
  location TEXT,
  website TEXT,
  twitter TEXT,
  linkedin TEXT,
  company TEXT,
  skills TEXT DEFAULT '[]',
  interests TEXT DEFAULT '[]',
  updated_at TEXT DEFAULT (datetime('now'))
);

-- REPOSITORIES TABLE
CREATE TABLE IF NOT EXISTS repositories (
  id TEXT PRIMARY KEY,
  github_id INTEGER UNIQUE NOT NULL,
  name TEXT NOT NULL,
  owner TEXT NOT NULL,
  description TEXT,
  language TEXT,
  stars INTEGER DEFAULT 0,
  forks INTEGER DEFAULT 0,
  created_at TEXT DEFAULT (datetime('now'))
);

-- ACTIVITIES TABLE
CREATE TABLE IF NOT EXISTS activities (
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  activity_type TEXT NOT NULL,
  repository_id TEXT REFERENCES repositories(id) ON DELETE SET NULL,
  title TEXT NOT NULL,
  description TEXT,
  link TEXT,
  metadata TEXT DEFAULT '{}',
  xp_earned INTEGER DEFAULT 0,
  created_at TEXT DEFAULT (datetime('now'))
);

-- ISSUES TABLE
CREATE TABLE IF NOT EXISTS issues (
  id TEXT PRIMARY KEY,
  github_id INTEGER UNIQUE NOT NULL,
  repository_id TEXT NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
  number INTEGER NOT NULL,
  title TEXT NOT NULL,
  body TEXT,
  state TEXT NOT NULL,
  labels TEXT DEFAULT '[]',
  creator_username TEXT NOT NULL,
  ai_complexity_score INTEGER,
  ai_match_score INTEGER,
  ai_analysis TEXT,
  created_at TEXT DEFAULT (datetime('now'))
);

-- BADGES TABLE
CREATE TABLE IF NOT EXISTS badges (
  id TEXT PRIMARY KEY,
  name TEXT UNIQUE NOT NULL,
  description TEXT,
  icon_url TEXT,
  xp_required INTEGER DEFAULT 0
);

-- USER BADGES TABLE
CREATE TABLE IF NOT EXISTS user_badges (
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  badge_id TEXT NOT NULL REFERENCES badges(id) ON DELETE CASCADE,
  awarded_at TEXT DEFAULT (datetime('now')),
  PRIMARY KEY (user_id, badge_id)
);

-- CONVERSATIONS TABLE
CREATE TABLE IF NOT EXISTS conversations (
  id TEXT PRIMARY KEY,
  is_group INTEGER DEFAULT 0,
  name TEXT,
  created_at TEXT DEFAULT (datetime('now'))
);

-- CONVERSATION PARTICIPANTS TABLE
CREATE TABLE IF NOT EXISTS conversation_participants (
  conversation_id TEXT NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  PRIMARY KEY (conversation_id, user_id)
);

-- MESSAGES TABLE
CREATE TABLE IF NOT EXISTS messages (
  id TEXT PRIMARY KEY,
  conversation_id TEXT NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
  sender_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  content TEXT NOT NULL,
  created_at TEXT DEFAULT (datetime('now'))
);

-- CONTRIBUTION STATS TABLE
CREATE TABLE IF NOT EXISTS contribution_stats (
  user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
  prs_opened INTEGER DEFAULT 0,
  prs_merged INTEGER DEFAULT 0,
  issues_opened INTEGER DEFAULT 0,
  commits_pushed INTEGER DEFAULT 0,
  stars_given INTEGER DEFAULT 0
);

-- REPUTATION HISTORY TABLE
CREATE TABLE IF NOT EXISTS reputation_history (
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  amount INTEGER NOT NULL,
  reason TEXT NOT NULL,
  created_at TEXT DEFAULT (datetime('now'))
);

-- INDEXES
CREATE INDEX IF NOT EXISTS idx_users_github_id ON users(github_id);
CREATE INDEX IF NOT EXISTS idx_users_github_username ON users(github_username);
CREATE INDEX IF NOT EXISTS idx_activities_user_id ON activities(user_id);
CREATE INDEX IF NOT EXISTS idx_activities_created_at ON activities(created_at);
CREATE INDEX IF NOT EXISTS idx_issues_repository_id ON issues(repository_id);
CREATE INDEX IF NOT EXISTS idx_messages_conversation_id ON messages(conversation_id);
CREATE INDEX IF NOT EXISTS idx_reputation_history_user_id ON reputation_history(user_id);

-- SEED BADGES
INSERT OR IGNORE INTO badges (id, name, description, xp_required) VALUES
  ('b1', 'First Merge', 'Merged your first pull request', 10),
  ('b2', 'Contributor', 'Earned 100 XP from open-source contributions', 100),
  ('b3', 'Maintainer', 'Earned 500 XP and created an issue/repository', 500),
  ('b4', 'Open Source Hero', 'Earned 2000 XP', 2000);
