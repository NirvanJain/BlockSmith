// Sidebar navigation items and their view identifiers
export type View =
  | "dashboard"
  | "chain-explorer"
  | "submit-contribution"
  | "leaderboard"
  | "repositories"
  | "verify"
  | "audit-log"
  | "api-keys";

export interface Block {
  index: number;
  timestamp: string;
  contributor: string;
  repository: string;
  contribution_type: string;
  contribution_link: string;
  previous_hash: string;
  hash: string;
}

export interface ContributionModel {
  id: number;
  github_username: string;
  repository: string;
  contribution_type: string;
  contribution_link: string;
  verified: boolean;
  created_at: string;
}

export interface LeaderboardEntry {
  rank: number;
  github_username: string;
  reputation_score: number;
  verified_contributions: number;
  total_contributions: number;
}

export interface RepoEntry {
  name: string;
  owner: string;
  tracked_since: string;
  contributions: number;
  status: "active" | "syncing" | "paused";
}

export interface LogEntry {
  id: number;
  time: string;
  level: "info" | "ok" | "warn" | "err";
  message: string;
}
