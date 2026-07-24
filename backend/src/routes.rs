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
use uuid::Uuid;

use crate::database::{self, db::DbPool};
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
    pool: &DbPool,
) -> Result<Uuid, (StatusCode, String)> {
    let auth_header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Invalid Authorization header format".to_string(),
        ));
    }

    let token = &auth_header[7..];
    let claims = crate::auth::jwt::verify_jwt(token)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid token: {}", e)))?;

    let user = database::users::find_by_clerk_id(pool, &claims.sub)
        .await
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(user.id)
}

async fn get_me(
    headers: axum::http::HeaderMap,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let user_id = authenticate_user(&headers, &pool).await?;

    let user = database::users::find_by_clerk_id(
        &pool,
        &crate::auth::jwt::verify_jwt(
            &headers
                .get("authorization")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("Bearer ")
                .trim_start_matches("Bearer "),
        )
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?
        .sub
        .as_str(),
    )
    .await
    .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let profile = database::profiles::get(&pool, user_id).await;

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
        "bio": profile.as_ref().and_then(|p| p.bio.clone()).unwrap_or_default(),
        "company": profile.as_ref().and_then(|p| p.company.clone()).unwrap_or_default(),
        "location": profile.as_ref().and_then(|p| p.location.clone()).unwrap_or_default(),
        "website": profile.as_ref().and_then(|p| p.website.clone()).unwrap_or_default(),
        "skills": profile.as_ref().map(|p| p.skills.clone()).unwrap_or_default(),
        "interests": profile.as_ref().map(|p| p.interests.clone()).unwrap_or_default(),
    })))
}

async fn get_feed(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let feed = database::activities::get_feed(&pool, 50).await;

    if feed.is_empty() {
        return Ok(Json(serde_json::json!([
            {
                "id": "act_1",
                "activity_type": "pr_merged",
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
                "activity_type": "badge_awarded",
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

    let list: Vec<serde_json::Value> = feed
        .into_iter()
        .map(|item| {
            serde_json::json!({
                "id": item.id,
                "activity_type": item.activity_type,
                "title": item.title,
                "description": item.description,
                "link": item.link,
                "xp_earned": item.xp_earned,
                "created_at": item.created_at,
                "repository": item.repository,
                "actor": {
                    "name": item.author_name,
                    "avatar_url": item.author_avatar,
                    "github_username": item.author_username,
                }
            })
        })
        .collect();

    Ok(Json(serde_json::json!(list)))
}

async fn get_leaderboard(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let leaderboard = database::leaderboard::get_top(&pool, 100).await;

    if leaderboard.is_empty() {
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

    let list: Vec<serde_json::Value> = leaderboard
        .into_iter()
        .map(|entry| {
            serde_json::json!({
                "rank": entry.rank,
                "id": entry.user_id,
                "name": entry.name,
                "github_username": entry.github_username,
                "avatar_url": entry.avatar_url,
                "reputation_score": entry.reputation_score,
                "xp": entry.xp,
                "level": entry.level,
            })
        })
        .collect();

    Ok(Json(serde_json::json!(list)))
}

async fn get_profile(
    Path(username): Path<String>,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let user = database::users::find_by_github_username(&pool, &username).await;

    let user = match user {
        Some(u) => u,
        None => {
            return Ok(Json(serde_json::json!({
                "user": {
                    "id": uuid::Uuid::new_v4(),
                    "name": format!("Mock {}", username),
                    "github_username": username,
                    "avatar_url": "https://avatars.githubusercontent.com/u/5832347?v=4",
                    "reputation_score": 120,
                    "xp": 120,
                    "level": 2,
                    "bio": "Passionate developer exploring BlockSmith",
                    "company": "OpenSource",
                    "location": "Earth",
                    "website": "https://blocksmith.dev",
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

    let profile = database::profiles::get(&pool, user.id).await;
    let badges = database::badges::get_user_badges(&pool, user.id).await;
    let activities = database::activities::get_user_activities(&pool, user.id, 10).await;

    Ok(Json(serde_json::json!({
        "user": {
            "id": user.id,
            "name": user.name,
            "github_username": user.github_username,
            "avatar_url": user.avatar_url,
            "reputation_score": user.reputation_score,
            "xp": user.xp,
            "level": user.level,
            "bio": profile.as_ref().and_then(|p| p.bio.clone()).unwrap_or_default(),
            "company": profile.as_ref().and_then(|p| p.company.clone()).unwrap_or_default(),
            "location": profile.as_ref().and_then(|p| p.location.clone()).unwrap_or_default(),
            "website": profile.as_ref().and_then(|p| p.website.clone()).unwrap_or_default(),
            "skills": profile.as_ref().map(|p| p.skills.clone()).unwrap_or_default(),
            "interests": profile.as_ref().map(|p| p.interests.clone()).unwrap_or_default(),
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
    let issues = database::issues::get_discovery(&pool, 50).await;

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

    let list: Vec<serde_json::Value> = issues
        .into_iter()
        .map(|item| {
            serde_json::json!({
                "id": item.issue_id,
                "title": item.title,
                "body": item.body,
                "state": item.state,
                "labels": item.labels,
                "creator_username": item.creator_username,
                "repo": {
                    "name": item.repository_name,
                    "owner": item.repository_owner,
                },
                "ai_complexity_score": item.ai_complexity_score.unwrap_or(5),
                "ai_match_score": item.ai_match_score.unwrap_or(75),
                "ai_analysis": item.ai_analysis.unwrap_or_else(|| "Complexity score fits your technical skills.".to_string()),
            })
        })
        .collect();

    Ok(Json(serde_json::json!(list)))
}

async fn get_conversations(
    headers: axum::http::HeaderMap,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let user_id = authenticate_user(&headers, &pool).await?;
    let conversations = database::conversations::get_user_conversations(&pool, user_id).await;

    let list: Vec<serde_json::Value> = conversations
        .into_iter()
        .map(|c| {
            serde_json::json!({
                "id": c.conversation.id,
                "is_group": c.conversation.is_group,
                "name": c.conversation.name,
                "participants": c.participants.into_iter().map(|p| {
                    serde_json::json!({
                        "user_id": p.user_id,
                        "name": p.name,
                        "github_username": p.github_username,
                        "avatar_url": p.avatar_url,
                    })
                }).collect::<Vec<_>>(),
                "created_at": c.conversation.created_at,
            })
        })
        .collect();

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

    let recipient = database::users::find_by_github_username(&pool, &req.recipient_username)
        .await
        .ok_or((StatusCode::NOT_FOUND, "Recipient not found".to_string()))?;

    if let Some(existing) = database::conversations::find_existing_dm(&pool, sender_id, recipient.id).await {
        return Ok(Json(serde_json::json!({ "id": existing.id })));
    }

    let convo = database::conversations::create(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    database::conversations::add_participant(&pool, convo.id, sender_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    database::conversations::add_participant(&pool, convo.id, recipient.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({ "id": convo.id })))
}

async fn get_messages(
    Path(conv_id): Path<Uuid>,
    headers: axum::http::HeaderMap,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let _user_id = authenticate_user(&headers, &pool).await?;
    let messages = database::messages::get_messages(&pool, conv_id).await;

    let list: Vec<serde_json::Value> = messages
        .into_iter()
        .map(|m| {
            serde_json::json!({
                "id": m.id,
                "sender_id": m.sender_id,
                "sender_name": m.sender_name,
                "sender_avatar": m.sender_avatar,
                "content": m.content,
                "created_at": m.created_at,
            })
        })
        .collect();

    Ok(Json(serde_json::json!(list)))
}
