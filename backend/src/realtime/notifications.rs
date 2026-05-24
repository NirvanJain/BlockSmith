use chrono::Utc;

use crate::realtime::events::{
    RealtimeEvent,
    RealtimeEventType,
};

pub fn create_block_notification(
    username: &str,
    repository: &str,
) -> RealtimeEvent {
    RealtimeEvent {
        event_type:
            RealtimeEventType::BlockCreated,

        message: format!(
            "{} created a verified contribution block",
            username
        ),

        username: username.to_string(),

        repository:
            repository.to_string(),

        timestamp:
            Utc::now().to_rfc3339(),
    }
}

pub fn create_pr_notification(
    username: &str,
    repository: &str,
) -> RealtimeEvent {
    RealtimeEvent {
        event_type:
            RealtimeEventType::PullRequestMerged,

        message: format!(
            "{} merged a PR in {}",
            username,
            repository
        ),

        username: username.to_string(),

        repository:
            repository.to_string(),

        timestamp:
            Utc::now().to_rfc3339(),
    }
}