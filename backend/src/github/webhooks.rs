use axum::{
    extract::Json,
    http::StatusCode,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookRepository {
    pub full_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookSender {
    pub login: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestInfo {
    pub html_url: String,
    pub merged: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestWebhook {
    pub action: String,
    pub repository: WebhookRepository,
    pub sender: WebhookSender,
    pub pull_request: PullRequestInfo,
}

pub async fn github_webhook_handler(
    Json(payload): Json<PullRequestWebhook>,
) -> Result<String, StatusCode> {
    println!(
        "Webhook received from repo: {}",
        payload.repository.full_name
    );

    println!(
        "Contributor: {}",
        payload.sender.login
    );

    println!(
        "Action: {}",
        payload.action
    );

    // Example:
    // If PR merged → create blockchain block
    if payload.action == "closed"
        && payload.pull_request.merged
            == Some(true)
    {
        println!("Merged PR verified!");
    }

    Ok("Webhook processed".to_string())
}