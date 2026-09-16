export interface Env {
  DB: D1Database;
  JWT_SECRET: string;
  GITHUB_CLIENT_ID: string;
  GITHUB_CLIENT_SECRET: string;
  GITHUB_REDIRECT_URI: string;
}

export type Variables = {
  userId: string;
};

export interface UserRow {
  id: string;
  githubId: string | null;
  githubUsername: string | null;
  githubAccessToken: string | null;
  email: string | null;
  name: string | null;
  avatarUrl: string | null;
  reputationScore: number;
  trustScore: number;
  totalContributions: number;
  xp: number;
  level: number;
  createdAt: string;
  updatedAt: string;
}

export interface ProfileRow {
  userId: string;
  bio: string | null;
  location: string | null;
  website: string | null;
  twitter: string | null;
  linkedin: string | null;
  company: string | null;
  skills: string;
  interests: string;
  updatedAt: string;
}

export interface FeedItem {
  id: string;
  authorName: string | null;
  authorUsername: string | null;
  authorAvatar: string | null;
  activityType: string;
  title: string;
  description: string | null;
  link: string | null;
  repository: string | null;
  xpEarned: number;
  createdAt: string;
}

export interface LeaderboardEntry {
  rank: number;
  userId: string;
  name: string | null;
  githubUsername: string | null;
  avatarUrl: string | null;
  reputationScore: number;
  xp: number;
  level: number;
  totalContributions: number;
}

export interface DiscoveryItem {
  issueId: string;
  title: string;
  body: string | null;
  state: string;
  labels: string;
  creatorUsername: string;
  aiComplexityScore: number | null;
  aiMatchScore: number | null;
  aiAnalysis: string | null;
  repositoryName: string;
  repositoryOwner: string;
  createdAt: string;
}
