use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookRepository {
    pub full_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSender {
    pub login: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestWebhook {
    pub action: String,
    pub repository: WebhookRepository,
    pub sender: WebhookSender,
}