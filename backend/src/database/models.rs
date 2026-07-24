use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ===== Core User =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub clerk_user_id: String,
    pub github_username: Option<String>,
    pub github_id: Option<String>,
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

// ===== Profile =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProfileRow {
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

// ===== Followers =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FollowerRow {
    pub follower_id: Uuid,
    pub following_id: Uuid,
    pub created_at: DateTime<Utc>,
}

// ===== Repository =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RepositoryRow {
    pub id: Uuid,
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

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ActivityRow {
    pub id: Uuid,
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

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct IssueRow {
    pub id: Uuid,
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

// ===== Pull Requests =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PullRequestRow {
    pub id: Uuid,
    pub github_id: i64,
    pub repository_id: Uuid,
    pub number: i32,
    pub title: String,
    pub state: String,
    pub merged: bool,
    pub creator_username: String,
    pub created_at: DateTime<Utc>,
}

// ===== Comments =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CommentRow {
    pub id: Uuid,
    pub activity_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

// ===== Reactions =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReactionRow {
    pub id: Uuid,
    pub activity_id: Uuid,
    pub user_id: Uuid,
    pub reaction_type: String,
    pub created_at: DateTime<Utc>,
}

// ===== Bookmarks =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BookmarkRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub activity_id: Uuid,
    pub created_at: DateTime<Utc>,
}

// ===== Badges =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BadgeRow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub xp_required: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserBadgeRow {
    pub user_id: Uuid,
    pub badge_id: Uuid,
    pub awarded_at: DateTime<Utc>,
}

// ===== Achievements =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AchievementRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub unlocked_at: DateTime<Utc>,
}

// ===== Leaderboard =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LeaderboardRow {
    pub user_id: Uuid,
    pub rank: Option<i32>,
    pub reputation_score: i32,
    pub updated_at: DateTime<Utc>,
}

// ===== Teams =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamRow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamMemberRow {
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}

// ===== Conversations & Messages =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConversationRow {
    pub id: Uuid,
    pub is_group: bool,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConversationParticipantRow {
    pub conversation_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MessageRow {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

// ===== Notifications =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NotificationRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub body: String,
    pub link: Option<String>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

// ===== Mentorships =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MentorshipRow {
    pub id: Uuid,
    pub mentor_id: Uuid,
    pub mentee_id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

// ===== Endorsements =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EndorsementRow {
    pub id: Uuid,
    pub endorser_id: Uuid,
    pub endorsee_id: Uuid,
    pub skill: String,
    pub created_at: DateTime<Utc>,
}

// ===== Recruiter Profiles =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RecruiterProfileRow {
    pub user_id: Uuid,
    pub company_name: String,
    pub company_website: Option<String>,
    pub subscription_tier: String,
    pub created_at: DateTime<Utc>,
}

// ===== Reports =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReportRow {
    pub id: Uuid,
    pub reporter_id: Uuid,
    pub reported_user_id: Uuid,
    pub content_type: String,
    pub content_id: Option<Uuid>,
    pub reason: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

// ===== Contribution Stats =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ContributionStatsRow {
    pub user_id: Uuid,
    pub prs_opened: i32,
    pub prs_merged: i32,
    pub issues_opened: i32,
    pub commits_pushed: i32,
    pub stars_given: i32,
}

// ===== Reputation History =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReputationHistoryRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: i32,
    pub reason: String,
    pub created_at: DateTime<Utc>,
}

// ===== Repository Tracking =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RepositoryTrackingRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub repository_id: Uuid,
    pub last_synced_at: Option<DateTime<Utc>>,
}

// ===== Audit Logs =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuditLogRow {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ===== Moderation Logs =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ModerationLogRow {
    pub id: Uuid,
    pub moderator_id: Option<Uuid>,
    pub action: String,
    pub target_id: Option<Uuid>,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ===== Events =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EventRow {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

// ===== Communities =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CommunityRow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
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
    pub followers_count: i64,
    pub following_count: i64,
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
