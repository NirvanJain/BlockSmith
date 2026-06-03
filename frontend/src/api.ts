import type { User, Activity, LeaderboardEntry, DiscoveryIssue, Conversation, Message, UserProfile } from './types';

const API_BASE = import.meta.env.VITE_API_URL || 'http://localhost:8000';
export const WS_BASE = API_BASE.replace(/^http/, 'ws');

function authHeaders(token: string | null): Record<string, string> {
  return token ? { Authorization: `Bearer ${token}` } : {};
}

async function get<T>(path: string, token?: string | null): Promise<T> {
  const resp = await fetch(`${API_BASE}${path}`, {
    headers: { 'Content-Type': 'application/json', ...authHeaders(token ?? null) },
  });
  if (!resp.ok) throw new Error(`${resp.status} ${resp.statusText}`);
  return resp.json();
}

async function post<T>(path: string, body: unknown, token: string | null): Promise<T> {
  const resp = await fetch(`${API_BASE}${path}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...authHeaders(token) },
    body: JSON.stringify(body),
  });
  if (!resp.ok) throw new Error(`${resp.status} ${resp.statusText}`);
  return resp.json();
}

// Requires auth
export function fetchMe(token: string | null): Promise<User> {
  if (!token) return Promise.reject(new Error('No auth token'));
  return get<User>('/api/v1/me', token);
}

// Public — no auth required by backend
export function fetchFeed(): Promise<Activity[]> {
  return get<Activity[]>('/api/v1/feed');
}

export function fetchLeaderboard(): Promise<LeaderboardEntry[]> {
  return get<LeaderboardEntry[]>('/api/v1/leaderboard');
}

export function fetchDiscovery(): Promise<DiscoveryIssue[]> {
  return get<DiscoveryIssue[]>('/api/v1/discovery');
}

export function fetchProfile(username: string, token: string | null): Promise<UserProfile> {
  return get<UserProfile>(`/api/v1/profile/${encodeURIComponent(username)}`, token);
}

// Chat — requires auth
export function fetchConversations(token: string | null): Promise<Conversation[]> {
  if (!token) return Promise.reject(new Error('No auth token'));
  return get<Conversation[]>('/api/v1/chat/conversations', token);
}

export function createConversation(token: string | null, recipient_username: string): Promise<{ id: string }> {
  if (!token) return Promise.reject(new Error('No auth token'));
  return post<{ id: string }>('/api/v1/chat/conversations', { recipient_username }, token);
}

export function fetchMessages(token: string | null, conversationId: string): Promise<Message[]> {
  if (!token) return Promise.reject(new Error('No auth token'));
  return get<Message[]>(`/api/v1/chat/conversations/${conversationId}/messages`, token);
}

// Compatibility aliases
export const getMe = fetchMe;
export const getFeed = fetchFeed;
export const getLeaderboard = fetchLeaderboard;
export const getProfile = fetchProfile;
export const getDiscovery = fetchDiscovery;
