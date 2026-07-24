use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use svix::webhooks::Webhook;

use crate::AppState;
use crate::database;

#[derive(Debug, Deserialize)]
pub struct ClerkWebhookEvent {
    pub data: Value,
    #[serde(rename = "type")]
    pub event_type: String,
}

pub async fn handle_clerk_webhook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: String,
) -> Result<StatusCode, (StatusCode, String)> {
    let secret = std::env::var("CLERK_WEBHOOK_SECRET").unwrap_or_default();

    if !secret.is_empty() {
        let wh = Webhook::new(&secret).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse webhook secret: {}", e),
            )
        })?;

        wh.verify(body.as_bytes(), &headers).map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                format!("Invalid webhook signature: {}", e),
            )
        })?;
    }

    let event: ClerkWebhookEvent = serde_json::from_str(&body).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid JSON payload: {}", e),
        )
    })?;

    match event.event_type.as_str() {
        "user.created" | "user.updated" => {
            let user_id = event.data.get("id").and_then(|v| v.as_str()).ok_or((
                StatusCode::BAD_REQUEST,
                "Missing user id in webhook data".to_string(),
            ))?;

            let email = event
                .data
                .get("email_addresses")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
                .and_then(|e| e.get("email_address"))
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let first_name = event
                .data
                .get("first_name")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let last_name = event
                .data
                .get("last_name")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let full_name = format!("{} {}", first_name, last_name).trim().to_string();
            let avatar_url = event
                .data
                .get("image_url")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let mut github_username: Option<String> = None;

            if let Some(accounts) = event.data.get("external_accounts").and_then(|v| v.as_array()) {
                for acct in accounts {
                    if acct.get("provider").and_then(|v| v.as_str()) == Some("oauth_github") {
                        github_username = acct
                            .get("username")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        break;
                    }
                }
            }

            let email_opt = if email.is_empty() { None } else { Some(email) };
            let name_opt = if full_name.is_empty() {
                None
            } else {
                Some(full_name.as_str())
            };
            let avatar_opt = if avatar_url.is_empty() {
                None
            } else {
                Some(avatar_url)
            };

            let user = database::users::upsert_from_clerk(&state.pool, user_id, email_opt, name_opt, avatar_opt)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Database error upserting user: {}", e),
                    )
                })?;

            if let Some(ref gh_user) = github_username {
                sqlx::query(
                    "UPDATE users SET github_username = $2 WHERE clerk_user_id = $1",
                )
                .bind(user_id)
                .bind(gh_user)
                .execute(&state.pool)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Database error updating github username: {}", e),
                    )
                })?;
            }

            if event.event_type == "user.created" {
                sqlx::query(
                    "INSERT INTO profiles (user_id, bio, company)
                     VALUES ($1, $2, $3)
                     ON CONFLICT (user_id) DO NOTHING",
                )
                .bind(user.id)
                .bind("OpenSource Contributor")
                .bind("")
                .execute(&state.pool)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Database error creating profile: {}", e),
                    )
                })?;

                sqlx::query(
                    "INSERT INTO contribution_stats (user_id)
                     VALUES ($1)
                     ON CONFLICT (user_id) DO NOTHING",
                )
                .bind(user.id)
                .execute(&state.pool)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Database error initializing stats: {}", e),
                    )
                })?;
            }
        }
        "user.deleted" => {
            let user_id = event.data.get("id").and_then(|v| v.as_str()).ok_or((
                StatusCode::BAD_REQUEST,
                "Missing user id in webhook data".to_string(),
            ))?;

            database::users::delete(&state.pool, user_id).await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database error deleting user: {}", e),
                )
            })?;
        }
        _ => {}
    }

    Ok(StatusCode::OK)
}
