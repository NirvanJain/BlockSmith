use axum::{
    routing::{get, post},
    Router,
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use serde::Deserialize;
use crate::realtime::websockets::{websocket_handler, WsState};

pub fn create_routes(ws_state: Arc<WsState>) -> Router<Pool<Postgres>> {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/webhooks/clerk", post(crate::auth::clerk_webhooks::handle_clerk_webhook))
        .route("/api/v1/webhooks/github", post(crate::github::webhooks::github_webhook_handler))
        .route("/api/v1/feed", get(get_feed))
        .route("/api/v1/leaderboard", get(get_leaderboard))
        .route("/api/v1/profile/:username", get(get_profile))
        .route("/api/v1/discovery", get(get_discovery))
        .route("/api/v1/chat/conversations", get(get_conversations).post(create_conversation))
        .route("/api/v1/chat/conversations/:id/messages", get(get_messages))
        .route("/api/v1/me", get(get_me))
        .route("/ws", get(websocket_handler).with_state(ws_state))
}

async fn health_check() -> &'static str {
    "OK"
}

async fn authenticate_user(
    headers: &axum::http::HeaderMap,
    pool: &Pool<Postgres>,
) -> Result<uuid::Uuid, (StatusCode, String)> {
    let auth_header = headers.get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::Unauthorized, "Missing Authorization header".to_string()))?;
        
    if !auth_header.starts_with("Bearer ") {
        return Err((StatusCode::Unauthorized, "Invalid Authorization header format".to_string()));
    }
    
    let token = &auth_header[7..];
    let claims = crate::auth::jwt::verify_jwt(token).await
        .map_err(|e| (StatusCode::Unauthorized, format!("Invalid token: {}", e)))?;
        
    let user_id = sqlx::query_scalar!(
        "SELECT id FROM users WHERE clerk_user_id = $1",
        claims.sub
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| (StatusCode::InternalServerError, e.to_string()))?
    .ok_or((StatusCode::NotFound, "User not found".to_string()))?;
    
    Ok(user_id)
}

async fn get_me(
    headers: axum::http::HeaderMap,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let user_id = authenticate_user(&headers, &pool).await?;
    let user = sqlx::query(
        r#"
        SELECT u.*, p.bio, p.company, p.location, p.website, p.skills, p.interests
        FROM users u
        LEFT JOIN profiles p ON p.user_id = u.id
        WHERE u.id = $1
        "#,
        user_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::InternalServerError, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "id": user.id,
        "clerk_user_id": user.clerk_user_id,
        "github_username": user.github_username,
        "github_id": user.github_id,
        "email": user.email,
        "name": user.name,
        "avatar_url": user.avatar_url,
        "reputation_score": user.reputation_score,
        "trust_score": user.trust_score,
        "total_contributions": user.total_contributions,
        "xp": user.xp,
        "level": user.level,
        "bio": user.bio,
        "company": user.company,
        "location": user.location,
        "website": user.website,
        "skills": user.skills.unwrap_or_default(),
        "interests": user.interests.unwrap_or_default(),
    })))
}

async fn get_feed(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let activities = sqlx::query(
        r#"
        SELECT a.*, u.name as actor_name, u.avatar_url as actor_avatar, u.github_username as actor_github
        FROM activities a
        JOIN users u ON a.user_id = u.id
        ORDER BY a.created_at DESC
        LIMIT 50
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::InternalServerError, e.to_string()))?;

    // If feed is empty, let's return some mock feed activities for first run/demo
    if activities.is_empty() {
        return Ok(Json(serde_json::json!([
            {
                "id": "act_1",
                "user_id": "user_mock",
                "activity_type": "pr_merged",
                "repository_id": null,
                "title": "Merged PR #21 in axum-rs/axum",
                "description": "feat: Add high performance WebSocket channels",
                "link": "https://github.com/tokio-rs/axum",
                "xp_earned": 10,
                "created_at": chrono::Utc::now(),
                "actor": {
                    "name": "The Octocat",
                    "avatar_url": "https://avatars.githubusercontent.com/u/5832347?v=4",
                    "github_username": "octocat",
                }
            },
            {
                "id": "act_2",
                "user_id": "user_mock",
                "activity_type": "badge_awarded",
                "repository_id": null,
                "title": "Unlocked 'Contributor' Badge",
                "description": "Earned 100 XP from verifiable contributions",
                "link": "",
                "xp_earned": 25,
                "created_at": chrono::Utc::now(),
                "actor": {
                    "name": "Nirvan Jain",
                    "avatar_url": "https://avatars.githubusercontent.com/u/5832347?v=4",
                    "github_username": "NirvanJain",
                }
            }
        ])));
    }

    let list: Vec<serde_json::Value> = activities.into_iter().map(|a| {
        serde_json::json!({
            "id": a.id,
            "user_id": a.user_id,
            "activity_type": a.activity_type,
            "repository_id": a.repository_id,
            "title": a.title,
            "description": a.description,
            "link": a.link,
            "metadata": a.metadata.unwrap_or_default(),
            "xp_earned": a.xp_earned,
            "created_at": a.created_at,
            "actor": {
                "name": a.actor_name,
                "avatar_url": a.actor_avatar,
                "github_username": a.actor_github,
            }
        })
    }).collect();

    Ok(Json(serde_json::json!(list)))
}

async fn get_leaderboard(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let users = sqlx::query(
        r#"
        SELECT id, name, github_username, avatar_url, reputation_score, xp, level
        FROM users
        ORDER BY reputation_score DESC
        LIMIT 100
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::InternalServerError, e.to_string()))?;

    if users.is_empty() {
        return Ok(Json(serde_json::json!([
            {
                "rank": 1,
                "id": "1",
                "name": "Nirvan Jain",
                "github_username": "NirvanJain",
                "avatar_url": "https://avatars.githubusercontent.com/u/5832347?v=4",
                "reputation_score": 1250,
                "xp": 1250,
                "level": 13,
            },
            {
                "rank": 2,
                "id": "2",
                "name": "The Octocat",
                "github_username": "octocat",
                "avatar_url": "https://avatars.githubusercontent.com/u/5832347?v=4",
                "reputation_score": 890,
                "xp": 890,
                "level": 9,
            }
        ])));
    }

    let list: Vec<serde_json::Value> = users.into_iter().enumerate().map(|(idx, u)| {
        serde_json::json!({
            "rank": idx + 1,
            "id": u.id,
            "name": u.name,
            "github_username": u.github_username,
            "avatar_url": u.avatar_url,
            "reputation_score": u.reputation_score,
            "xp": u.xp,
            "level": u.level,
        })
    }).collect();

    Ok(Json(serde_json::json!(list)))
}

async fn get_profile(
    Path(username): Path<String>,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let user = sqlx::query(
        r#"
        SELECT u.*, p.bio, p.company, p.location, p.website, p.skills, p.interests
        FROM users u
        LEFT JOIN profiles p ON p.user_id = u.id
        WHERE u.github_username = $1
        "#,
        username
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::InternalServerError, e.to_string()))?;

    let user = match user {
        Some(u) => u,
        None => {
            // Return mock profile if user not registered yet to allow browsing portfolio in demo
            return Ok(Json(serde_json::json!({
                "user": {
                    "id": uuid::Uuid::new_v4(),
                    "name": format!("Mock {}", username),
                    "github_username": username,
                    "avatar_url": "https://avatars.githubusercontent.com/u/5832347?v=4",
                    "reputation_score": 120,
                    "xp": 120,
                    "level": 2,
                    "bio": "Passionate developer exploring OpenHub",
                    "company": "OpenSource",
                    "location": "Earth",
                    "website": "https://openhub.dev",
                    "skills": ["Rust", "TypeScript", "React"],
                    "interests": ["Compilers", "Distributed Systems"],
                },
                "badges": [
                    {
                        "id": "b1",
                        "name": "First Merge",
                        "description": "Merged your first pull request",
                        "icon_url": null,
                    }
                ],
                "activities": [
                    {
                        "id": "a1",
                        "activity_type": "pr_merged",
                        "title": "Merged PR #12 in NirvanJain/BlockSmith",
                        "description": "feat: Add verification engine",
                        "link": "https://github.com/NirvanJain/BlockSmith",
                        "xp_earned": 10,
                        "created_at": chrono::Utc::now(),
                    }
                ]
            })));
        }
    };

    let badges = sqlx::query(
        r#"
        SELECT b.*
        FROM badges b
        JOIN user_badges ub ON ub.badge_id = b.id
        WHERE ub.user_id = $1
        "#,
        user.id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::InternalServerError, e.to_string()))?;

    let activities = sqlx::query(
        r#"
        SELECT * FROM activities
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT 10
        "#,
        user.id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::InternalServerError, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "user": {
            "id": user.id,
            "name": user.name,
            "github_username": user.github_username,
            "avatar_url": user.avatar_url,
            "reputation_score": user.reputation_score,
            "xp": user.xp,
            "level": user.level,
            "bio": user.bio,
            "company": user.company,
            "location": user.location,
            "website": user.website,
            "skills": user.skills.unwrap_or_default(),
            "interests": user.interests.unwrap_or_default(),
        },
        "badges": badges.into_iter().map(|b| serde_json::json!({
            "id": b.id,
            "name": b.name,
            "description": b.description,
            "icon_url": b.icon_url,
        })).collect::<Vec<_>>(),
        "activities": activities.into_iter().map(|a| serde_json::json!({
            "id": a.id,
            "activity_type": a.activity_type,
            "title": a.title,
            "description": a.description,
            "link": a.link,
            "xp_earned": a.xp_earned,
            "created_at": a.created_at,
        })).collect::<Vec<_>>(),
    })))
}

async fn get_discovery(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let issues = sqlx::query(
        r#"
        SELECT i.*, r.name as repo_name, r.owner as repo_owner
        FROM issues i
        JOIN repositories r ON i.repository_id = r.id
        ORDER BY i.created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::InternalServerError, e.to_string()))?;

    if issues.is_empty() {
        return Ok(Json(serde_json::json!([
            {
                "id": "iss_1",
                "number": 42,
                "title": "Optimize performance of Merkle tree generation",
                "body": "The current merkle tree implementation is single-threaded. We should parallelize it using Rayon.",
                "state": "open",
                "labels": ["performance", "rust"],
                "creator_username": "octocat",
                "repo": {
                    "name": "BlockSmith",
                    "owner": "NirvanJain",
                },
                "ai_complexity_score": 8,
                "ai_match_score": 95,
                "ai_analysis": "Highly matches your profile due to your experience with Rust and performance tuning.",
            },
            {
                "id": "iss_2",
                "number": 105,
                "title": "Add Clerk authentication middleware to Axum backend",
                "body": "We need to authenticate user endpoints using Clerk JWKS key validation.",
                "state": "open",
                "labels": ["good first issue", "backend", "auth"],
                "creator_username": "NirvanJain",
                "repo": {
                    "name": "BlockSmith",
                    "owner": "NirvanJain",
                },
                "ai_complexity_score": 4,
                "ai_match_score": 82,
                "ai_analysis": "Excellent choice for a developer focused on API building and backend architecture.",
            }
        ])));
    }

    let list: Vec<serde_json::Value> = issues.into_iter().map(|i| {
        serde_json::json!({
            "id": i.id,
            "number": i.number,
            "title": i.title,
            "body": i.body,
            "state": i.state,
            "labels": i.labels.unwrap_or_default(),
            "creator_username": i.creator_username,
            "repo": {
                "name": i.repo_name,
                "owner": i.repo_owner,
            },
            "ai_complexity_score": i.ai_complexity_score.unwrap_or(5),
            "ai_match_score": i.ai_match_score.unwrap_or(75),
            "ai_analysis": i.ai_analysis.unwrap_or_else(|| "Complexity score fits your technical skills.".to_string()),
        })
    }).collect();

    Ok(Json(serde_json::json!(list)))
}

async fn get_conversations(
    headers: axum::http::HeaderMap,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let user_id = authenticate_user(&headers, &pool).await?;

    let conversations = sqlx::query(
        r#"
        SELECT c.*, array_agg(u.github_username) as participants
        FROM conversations c
        JOIN conversation_participants cp ON cp.conversation_id = c.id
        JOIN users u ON cp.user_id = u.id
        WHERE c.id IN (
            SELECT conversation_id FROM conversation_participants WHERE user_id = $1
        )
        GROUP BY c.id
        "#,
        user_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::InternalServerError, e.to_string()))?;

    let list: Vec<serde_json::Value> = conversations.into_iter().map(|c| {
        serde_json::json!({
            "id": c.id,
            "is_group": c.is_group,
            "name": c.name,
            "participants": c.participants.unwrap_or_default(),
            "created_at": c.created_at,
        })
    }).collect();

    Ok(Json(serde_json::json!(list)))
}

#[derive(Debug, Deserialize)]
struct CreateConvRequest {
    recipient_username: String,
}

async fn create_conversation(
    headers: axum::http::HeaderMap,
    State(pool): State<Pool<Postgres>>,
    Json(req): Json<CreateConvRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let sender_id = authenticate_user(&headers, &pool).await?;

    let recipient_id = sqlx::query_scalar!(
        "SELECT id FROM users WHERE github_username = $1",
        req.recipient_username
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::InternalServerError, e.to_string()))?
    .ok_or((StatusCode::NotFound, "Recipient not found".to_string()))?;

    let existing_dm = sqlx::query_scalar!(
        r#"
        SELECT c.id FROM conversations c
        JOIN conversation_participants cp1 ON cp1.conversation_id = c.id
        JOIN conversation_participants cp2 ON cp2.conversation_id = c.id
        WHERE c.is_group = FALSE AND cp1.user_id = $1 AND cp2.user_id = $2
        "#,
        sender_id,
        recipient_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::InternalServerError, e.to_string()))?;

    if let Some(dm_id) = existing_dm {
        return Ok(Json(serde_json::json!({ "id": dm_id })));
    }

    let conv_id = sqlx::query_scalar!(
        "INSERT INTO conversations (is_group) VALUES (FALSE) RETURNING id"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::InternalServerError, e.to_string()))?;

    sqlx::query(
        "INSERT INTO conversation_participants (conversation_id, user_id) VALUES ($1, $2), ($1, $3)",
        conv_id,
        sender_id,
        recipient_id
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::InternalServerError, e.to_string()))?;

    Ok(Json(serde_json::json!({ "id": conv_id })))
}

async fn get_messages(
    Path(conv_id): Path<uuid::Uuid>,
    headers: axum::http::HeaderMap,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let _user_id = authenticate_user(&headers, &pool).await?;

    let messages = sqlx::query(
        r#"
        SELECT m.*, u.github_username as sender_username, u.avatar_url as sender_avatar
        FROM messages m
        JOIN users u ON m.sender_id = u.id
        WHERE m.conversation_id = $1
        ORDER BY m.created_at ASC
        "#,
        conv_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::InternalServerError, e.to_string()))?;

    let list: Vec<serde_json::Value> = messages.into_iter().map(|m| {
        serde_json::json!({
            "id": m.id,
            "conversation_id": m.conversation_id,
            "sender_id": m.sender_id,
            "sender_username": m.sender_username,
            "sender_avatar": m.sender_avatar,
            "content": m.content,
            "created_at": m.created_at,
        })
    }).collect();

    Ok(Json(serde_json::json!(list)))
}