use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use serde_json::Value;
use sqlx::{Pool, Postgres};
use svix::webhooks::Webhook;

#[derive(Debug, Deserialize)]
pub struct ClerkWebhookEvent {
    pub data: Value,
    #[serde(rename = "type")]
    pub event_type: String,
}

pub async fn handle_clerk_webhook(
    State(pool): State<Pool<Postgres>>,
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

            let email = event.data.get("email_addresses")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
                .and_then(|e| e.get("email_address"))
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let first_name = event.data.get("first_name").and_then(|v| v.as_str()).unwrap_or("");
            let last_name = event.data.get("last_name").and_then(|v| v.as_str()).unwrap_or("");
            let full_name = format!("{} {}", first_name, last_name).trim().to_string();
            let avatar_url = event.data.get("image_url").and_then(|v| v.as_str()).unwrap_or("");

            let mut github_username: Option<String> = None;
            let mut github_id: Option<String> = None;

            if let Some(accounts) = event.data.get("external_accounts").and_then(|v| v.as_array()) {
                for acct in accounts {
                    if acct.get("provider").and_then(|v| v.as_str()) == Some("oauth_github") {
                        github_username = acct.get("username").and_then(|v| v.as_str()).map(|s| s.to_string());
                        
                        // Handle github id being integer or string
                        github_id = acct.get("uid").map(|v| {
                            if v.is_string() {
                                v.as_str().unwrap().to_string()
                            } else if v.is_number() {
                                v.as_i64().unwrap().to_string()
                            } else {
                                "".to_string()
                            }
                        });
                        break;
                    }
                }
            }

            sqlx::query(
                r#"
                INSERT INTO users (clerk_user_id, github_username, github_id, email, name, avatar_url)
                VALUES ($1, $2, $3, $4, $5, $6)
                ON CONFLICT (clerk_user_id) DO UPDATE
                SET github_username = COALESCE(EXCLUDED.github_username, users.github_username),
                    github_id = COALESCE(EXCLUDED.github_id, users.github_id),
                    email = EXCLUDED.email,
                    name = EXCLUDED.name,
                    avatar_url = EXCLUDED.avatar_url,
                    updated_at = CURRENT_TIMESTAMP
                "#,
            )
            .bind(user_id)
            .bind(github_username)
            .bind(github_id)
            .bind(if email.is_empty() { None } else { Some(email) })
            .bind(if full_name.is_empty() { None } else { Some(full_name) })
            .bind(if avatar_url.is_empty() { None } else { Some(avatar_url) })
            .execute(&pool)
            .await
            .map_err(|e| (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error upserting user: {}", e),
            ))?;

            if event.event_type == "user.created" {
                let user_uuid = sqlx::query_scalar!(
                    "SELECT id FROM users WHERE clerk_user_id = $1",
                    user_id
                )
                .fetch_one(&pool)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Database error fetching user: {}", e),
                    )
                })?;

                sqlx::query(
                    r#"
                    INSERT INTO profiles (user_id, bio, company)
                    VALUES ($1, $2, $3)
                    ON CONFLICT (user_id) DO NOTHING
                    "#,
                )
                .bind(user_uuid)
                .bind("OpenSource Contributor")
                .bind("")
                .execute(&pool)
                .await
                .map_err(|e| (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database error creating profile: {}", e),
                ))?;
                
                // Initialize user stats
                sqlx::query(
                    r#"
                    INSERT INTO contribution_stats (user_id)
                    VALUES ($1)
                    ON CONFLICT (user_id) DO NOTHING
                    "#,
                )
                .bind(user_uuid)
                .execute(&pool)
                .await
                .map_err(|e| (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database error initializing stats: {}", e),
                ))?;
            }
        }
        "user.deleted" => {
            let user_id = event.data.get("id").and_then(|v| v.as_str()).ok_or((
                StatusCode::BAD_REQUEST,
                "Missing user id in webhook data".to_string(),
            ))?;

            let user_uuid = sqlx::query_scalar!(
                "SELECT id FROM users WHERE clerk_user_id = $1",
                user_id
            )
            .fetch_one(&pool)
            .await
            .map_err(|e| (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error fetching user: {}", e),
            ))?;

            sqlx::query!(
                "DELETE FROM users WHERE clerk_user_id = $1",
                user_id
            )
            .execute(&pool)
            .await
            .map_err(|e| {
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
