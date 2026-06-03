use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sqlx::{Pool, Postgres};

type HmacSha256 = Hmac<Sha256>;

fn verify_github_signature(secret: &str, signature: &str, body: &[u8]) -> bool {
    if !signature.starts_with("sha256=") {
        return false;
    }
    let signature_hex = &signature[7..];
    let signature_bytes = match hex::decode(signature_hex) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    let mut mac = match HmacSha256::new_from_slice(secret.as_bytes()) {
        Ok(m) => m,
        Err(_) => return false,
    };
    mac.update(body);
    mac.verify_slice(&signature_bytes).is_ok()
}

pub async fn github_webhook_handler(
    State(pool): State<Pool<Postgres>>,
    headers: HeaderMap,
    body: String,
) -> Result<StatusCode, (StatusCode, String)> {
    let secret = std::env::var("GITHUB_WEBHOOK_SECRET").unwrap_or_default();
    
    if !secret.is_empty() {
        let signature = headers
            .get("x-hub-signature-256")
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing signature".to_string()))?;

        if !verify_github_signature(&secret, signature, body.as_bytes()) {
            return Err((StatusCode::UNAUTHORIZED, "Invalid signature".to_string()));
        }
    }

    let payload: serde_json::Value = serde_json::from_str(&body).map_err(|e| {
        (StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", e))
    })?;

    let event_type = headers
        .get("x-github-event")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("ping");

    if event_type == "ping" {
        return Ok(StatusCode::OK);
    }

    if event_type == "pull_request" {
        let action = payload.get("action").and_then(|v| v.as_str()).unwrap_or("");
        let pr = payload.get("pull_request");
        
        if action == "closed" {
            let merged = pr.and_then(|p| p.get("merged")).and_then(|v| v.as_bool()).unwrap_or(false);
            if merged {
                let username = payload.get("sender").and_then(|v| v.get("login")).and_then(|v| v.as_str()).unwrap_or("");
                let repo_name = payload.get("repository").and_then(|v| v.get("name")).and_then(|v| v.as_str()).unwrap_or("");
                let repo_owner = payload.get("repository").and_then(|v| v.get("owner")).and_then(|v| v.as_str()).unwrap_or("");
                let pr_title = pr.and_then(|p| p.get("title")).and_then(|v| v.as_str()).unwrap_or("Pull Request");
                let pr_url = pr.and_then(|p| p.get("html_url")).and_then(|v| v.as_str()).unwrap_or("");
                let github_repo_id = payload.get("repository").and_then(|v| v.get("id")).and_then(|v| v.as_i64()).unwrap_or(0);

                let user_row = sqlx::query("SELECT id, reputation_score, xp, level FROM users WHERE github_username = $1")
    .bind(username)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
let user_row = match user_row {
    Some(row) => {
        // Map row to struct manually
        let id: i32 = row.try_get("id").unwrap();
        let reputation_score: i32 = row.try_get("reputation_score").unwrap();
        let xp: i32 = row.try_get("xp").unwrap();
        let level: i32 = row.try_get("level").unwrap();
        // create a temporary struct for usage
        struct UserRow { id: i32, reputation_score: i32, xp: i32, level: i32 }
        UserRow { id, reputation_score, xp, level }
    },
    None => return Err((StatusCode::NOT_FOUND, "User not found".to_string()))
};
                    
                    let repo_uuid = sqlx::query_scalar!(
                        r#"
                        INSERT INTO repositories (github_id, name, owner)
                        VALUES ($1, $2, $3)
                        ON CONFLICT (github_id) DO UPDATE SET name = EXCLUDED.name, owner = EXCLUDED.owner
                        RETURNING id
                        "#,
                        github_repo_id,
                        repo_name,
                        repo_owner
                    )
                    .fetch_one(&pool)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    let xp_earned = 10;
                    let pr_num = pr.and_then(|p| p.get("number")).and_then(|v| v.as_i64()).unwrap_or(0);
                    let activity_title = format!("Merged PR #{} in {}/{}", pr_num, repo_owner, repo_name);
                    
                    sqlx::query!(
                        r#"
                        INSERT INTO activities (user_id, activity_type, repository_id, title, description, link, xp_earned)
                        VALUES ($1, $2, $3, $4, $5, $6, $7)
                        "#,
                        user_row.id,
                        "pr_merged",
                        repo_uuid,
                        activity_title,
                        pr_title,
                        pr_url,
                        xp_earned
                    )
                    .execute(&pool)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    let new_xp = user_row.xp + xp_earned;
                    let new_level = (new_xp / 100) + 1;
                    let new_reputation = user_row.reputation_score + xp_earned;

                    sqlx::query!(
                        r#"
                        UPDATE users
                        SET reputation_score = $1, xp = $2, level = $3, total_contributions = total_contributions + 1
                        WHERE id = $4
                        "#,
                        new_reputation,
                        new_xp,
                        new_level,
                        user_row.id
                    )
                    .execute(&pool)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    sqlx::query!(
                        r#"
                        INSERT INTO contribution_stats (user_id, prs_merged)
                        VALUES ($1, 1)
                        ON CONFLICT (user_id) DO UPDATE
                        SET prs_merged = contribution_stats.prs_merged + 1
                        "#,
                        user_row.id
                    )
                    .execute(&pool)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    sqlx::query!(
                        r#"
                        INSERT INTO reputation_history (user_id, amount, reason)
                        VALUES ($1, $2, $3)
                        "#,
                        user_row.id,
                        xp_earned,
                        format!("Merged PR: {}", pr_title)
                    )
                    .execute(&pool)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
            }
        }
    }

    Ok(StatusCode::OK)
}