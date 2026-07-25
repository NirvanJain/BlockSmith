use uuid::Uuid;

use super::db::DbPool;
use super::models::UserRow;

const SELECT_COLUMNS: &str = "id, clerk_user_id, github_username, github_id, github_access_token,
        email, name, avatar_url,
        reputation_score, trust_score, total_contributions, xp, level,
        created_at, updated_at";

pub async fn find_by_id(pool: &DbPool, user_id: Uuid) -> Option<UserRow> {
    sqlx::query_as::<_, UserRow>(
        &format!("SELECT {} FROM users WHERE id = $1", SELECT_COLUMNS),
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
}

pub async fn find_by_github_id(pool: &DbPool, github_id: &str) -> Option<UserRow> {
    sqlx::query_as::<_, UserRow>(
        &format!("SELECT {} FROM users WHERE github_id = $1", SELECT_COLUMNS),
    )
    .bind(github_id)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
}

pub async fn find_by_github_username(pool: &DbPool, github_username: &str) -> Option<UserRow> {
    sqlx::query_as::<_, UserRow>(
        &format!("SELECT {} FROM users WHERE github_username = $1", SELECT_COLUMNS),
    )
    .bind(github_username)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
}

/// Upsert a user from GitHub OAuth. The `github_id` is the primary identifier.
pub async fn upsert_from_github(
    pool: &DbPool,
    github_id: &str,
    github_username: &str,
    email: Option<&str>,
    name: Option<&str>,
    avatar_url: Option<&str>,
    access_token: &str,
) -> Result<UserRow, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        &format!(
            "INSERT INTO users (github_id, github_username, github_access_token, email, name, avatar_url)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (github_id) DO UPDATE
                SET github_username = EXCLUDED.github_username,
                    github_access_token = EXCLUDED.github_access_token,
                    email = COALESCE(EXCLUDED.email, users.email),
                    name = COALESCE(EXCLUDED.name, users.name),
                    avatar_url = COALESCE(EXCLUDED.avatar_url, users.avatar_url),
                    updated_at = CURRENT_TIMESTAMP
             RETURNING {}",
            SELECT_COLUMNS
        )
    )
    .bind(github_id)
    .bind(github_username)
    .bind(access_token)
    .bind(email)
    .bind(name)
    .bind(avatar_url)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &DbPool,
    user_id: Uuid,
    name: Option<&str>,
    email: Option<&str>,
    avatar_url: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE users SET
            name = COALESCE($2, name),
            email = COALESCE($3, email),
            avatar_url = COALESCE($4, avatar_url),
            updated_at = CURRENT_TIMESTAMP
         WHERE id = $1",
    )
    .bind(user_id)
    .bind(name)
    .bind(email)
    .bind(avatar_url)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_reputation(
    pool: &DbPool,
    user_id: Uuid,
    xp: i32,
    reputation_score: i32,
    level: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE users SET
            xp = $2,
            reputation_score = $3,
            level = $4,
            updated_at = CURRENT_TIMESTAMP
         WHERE id = $1",
    )
    .bind(user_id)
    .bind(xp)
    .bind(reputation_score)
    .bind(level)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_by_id(pool: &DbPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_by_github_id(pool: &DbPool, github_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE github_id = $1")
        .bind(github_id)
        .execute(pool)
        .await?;
    Ok(())
}
