use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;
use crate::auth::github_oauth;
use crate::database;

#[derive(Debug, Deserialize)]
pub struct CallbackParams {
    code: String,
    #[allow(dead_code)]
    state: Option<String>,
}

/// GET /api/v1/auth/github
/// Redirects the user to GitHub's OAuth authorization page.
pub async fn github_auth_initiate() -> Result<Redirect, (StatusCode, String)> {
    let state = uuid::Uuid::new_v4().to_string();
    let url = github_oauth::github_oauth_url(&state);
    Ok(Redirect::temporary(&url))
}

/// GET /api/v1/auth/github/callback?code=...&state=...
/// Handles the OAuth callback from GitHub, exchanges the code for a token,
/// fetches the user profile, and creates/updates the user in the database.
pub async fn github_auth_callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CallbackParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // 1. Exchange the authorization code for an access token
    let access_token = github_oauth::exchange_code_for_token(&params.code)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    // 2. Fetch the GitHub user profile
    let gh_user = github_oauth::fetch_github_profile(&access_token)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    // 3. Try to fetch the primary email if not provided in profile
    let email = match gh_user.email {
        Some(ref e) if !e.is_empty() => gh_user.email.clone(),
        _ => github_oauth::fetch_github_email(&access_token)
            .await
            .unwrap_or(None),
    };

    let github_id = gh_user.id.to_string();

    // 4. Upsert the user in the database
    let user = database::users::upsert_from_github(
        &state.pool,
        &github_id,
        &gh_user.login,
        email.as_deref(),
        gh_user.name.as_deref(),
        gh_user.avatar_url.as_deref(),
        &access_token,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;

    // 5. Create a profile if this is a new user
    let _ = database::profiles::upsert(
        &state.pool,
        user.id,
        gh_user.bio.as_deref(),
        None, // location
        None, // website
        gh_user.company.as_deref(),
        &[],  // skills
        &[],  // interests
    )
    .await;

    // 6. Initialize contribution stats if new
    let _ = sqlx::query(
        "INSERT INTO contribution_stats (user_id)
         VALUES ($1)
         ON CONFLICT (user_id) DO NOTHING",
    )
    .bind(user.id)
    .execute(&state.pool)
    .await;

    // 7. Generate a JWT for the user
    let token = crate::auth::jwt::create_jwt(&user.id.to_string())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("JWT error: {}", e)))?;

    Ok(Json(serde_json::json!({
        "token": token,
        "user": {
            "id": user.id,
            "github_username": user.github_username,
            "github_id": user.github_id,
            "email": user.email,
            "name": user.name,
            "avatar_url": user.avatar_url,
            "reputation_score": user.reputation_score,
            "xp": user.xp,
            "level": user.level,
        }
    })))
}
