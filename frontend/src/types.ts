export interface User {
  id: string;
  clerk_user_id: string;
  github_username: string | null;
  github_id: string | null;
  email: string | null;
  name: string;
  avatar_url: string;
  reputation_score: number;
  trust_score: number;
  total_contributions: number;
  xp: number;
  level: number;
  bio?: string | null;
  company?: string | null;
  location?: string | null;
  website?: string | null;
  skills?: string[];
  interests?: string[];
}

export interface ActivityActor {
  name: string;
  avatar_url: string;
  github_username: string;
}

export interface Activity {
  id: string;
  user_id: string;
  activity_type: string;
  repository_id: string | null;
  title: string;
  description: string;
  link: string;
  metadata?: Record<string, unknown>;
  xp_earned: number;
  created_at: string;
  actor: ActivityActor;
}

export interface LeaderboardEntry {
  rank: number;
  id: string;
  name: string;
  github_username: string;
  avatar_url: string;
  reputation_score: number;
  xp: number;
  level: number;
}

export interface DiscoveryIssue {
  id: string;
  number: number;
  title: string;
  body: string;
  state: 'open' | 'closed';
  labels: string[];
  creator_username: string;
  repo: { name: string; owner: string };
  ai_complexity_score: number;
  ai_match_score: number;
  ai_analysis: string;
}

export interface Conversation {
  id: string;
  is_group: boolean;
  name: string | null;
  participants: string[];
  created_at: string;
}

export interface Message {
  id?: string;
  conversation_id?: string;
  sender_id?: string;
  sender_username: string;
  sender_avatar: string;
  content: string;
  created_at: string;
}

export interface UserProfile {
  user: {
    id: string;
    name: string;
    github_username: string;
    avatar_url: string;
    reputation_score: number;
    xp: number;
    level: number;
    bio: string | null;
    company: string | null;
    location: string | null;
    website: string | null;
    skills: string[];
    interests: string[];
  };
  badges: Array<{
    id: string;
    name: string;
    description: string;
    icon_url: string | null;
  }>;
  activities: Array<{
    id: string;
    activity_type: string;
    title: string;
    description: string;
    link: string;
    xp_earned: number;
    created_at: string;
  }>;
}
