use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ===== Core User =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRow {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub user_id: Uuid,
    pub github_username: Option<String>,
    pub github_id: Option<String>,
    pub github_access_token: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub reputation_score: i32,
    pub trust_score: i32,
    pub total_contributions: i32,
    pub xp: i32,
    pub level: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserRow {
    pub fn id(&self) -> Uuid {
        self.user_id
    }
}

// ===== Profile =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileRow {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub user_id: Uuid,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub twitter: Option<String>,
    pub linkedin: Option<String>,
    pub company: Option<String>,
    pub skills: Vec<String>,
    pub interests: Vec<String>,
    pub updated_at: DateTime<Utc>,
}

// ===== Repository =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryRow {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub repo_id: Uuid,
    pub github_id: i64,
    pub name: String,
    pub owner: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stars: i32,
    pub forks: i32,
    pub created_at: DateTime<Utc>,
}

// ===== Activities (Feed) =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityRow {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub activity_id: Uuid,
    pub user_id: Uuid,
    pub activity_type: String,
    pub repository_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub link: Option<String>,
    pub metadata: serde_json::Value,
    pub xp_earned: i32,
    pub created_at: DateTime<Utc>,
}

// ===== Issues =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueRow {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub issue_id: Uuid,
    pub github_id: i64,
    pub repository_id: Uuid,
    pub number: i32,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub labels: Vec<String>,
    pub creator_username: String,
    pub ai_complexity_score: Option<i32>,
    pub ai_match_score: Option<i32>,
    pub ai_analysis: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ===== Badges =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeRow {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub badge_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub xp_required: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBadgeRow {
    pub user_id: Uuid,
    pub badge_id: Uuid,
    pub awarded_at: DateTime<Utc>,
}

// ===== Leaderboard =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardRow {
    pub user_id: Uuid,
    pub rank: Option<i32>,
    pub reputation_score: i32,
    pub updated_at: DateTime<Utc>,
}

// ===== Conversations & Messages =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationRow {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub conversation_id: Uuid,
    pub is_group: bool,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRow {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub message_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

// ===== Contribution Stats =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionStatsRow {
    pub user_id: Uuid,
    pub prs_opened: i32,
    pub prs_merged: i32,
    pub issues_opened: i32,
    pub commits_pushed: i32,
    pub stars_given: i32,
}

// ===== Reputation History =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationHistoryRow {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub user_id: Uuid,
    pub amount: i32,
    pub reason: String,
    pub created_at: DateTime<Utc>,
}

// ===== Response DTOs (used by routes) =====

#[derive(Debug, Serialize)]
pub struct FeedItem {
    pub id: Uuid,
    pub author_name: String,
    pub author_username: String,
    pub author_avatar: Option<String>,
    pub activity_type: String,
    pub title: String,
    pub description: Option<String>,
    pub link: Option<String>,
    pub repository: Option<String>,
    pub xp_earned: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct LeaderboardEntry {
    pub rank: i32,
    pub user_id: Uuid,
    pub name: Option<String>,
    pub github_username: Option<String>,
    pub avatar_url: Option<String>,
    pub reputation_score: i32,
    pub xp: i32,
    pub level: i32,
    pub total_contributions: i32,
}

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub user: UserRow,
    pub profile: Option<ProfileRow>,
    pub badges: Vec<BadgeRow>,
    pub recent_activities: Vec<ActivityRow>,
}

#[derive(Debug, Serialize)]
pub struct DiscoveryItem {
    pub issue_id: Uuid,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub labels: Vec<String>,
    pub creator_username: String,
    pub ai_complexity_score: Option<i32>,
    pub ai_match_score: Option<i32>,
    pub ai_analysis: Option<String>,
    pub repository_name: String,
    pub repository_owner: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ConversationWithParticipants {
    pub conversation: ConversationRow,
    pub participants: Vec<ParticipantInfo>,
}

#[derive(Debug, Serialize)]
pub struct ParticipantInfo {
    pub user_id: Uuid,
    pub name: Option<String>,
    pub github_username: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub sender_name: Option<String>,
    pub sender_avatar: Option<String>,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
