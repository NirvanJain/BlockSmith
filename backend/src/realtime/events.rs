use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RealtimeEventType {
    ContributionVerified,
    PullRequestMerged,
    IssueClosed,
    BlockCreated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeEvent {
    pub event_type: RealtimeEventType,
    pub message: String,
    pub username: String,
    pub repository: String,
    pub timestamp: String,
}