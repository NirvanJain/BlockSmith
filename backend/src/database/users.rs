use sqlx::Row;
use uuid::Uuid;

use super::db::DbPool;
use super::models::UserRow;

pub async fn find_by_clerk_id(pool: &DbPool, clerk_user_id: &str) -> Option<UserRow> {
    sqlx::query_as::<_, UserRow>(
        "SELECT id, clerk_user_id, github_username, github_id, email, name, avatar_url,
                reputation_score, trust_score, total_contributions, xp, level,
                created_at, updated_at
         FROM users WHERE clerk_user_id = $1",
    )
    .bind(clerk_user_id)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
}

pub async fn find_by_github_username(pool: &DbPool, github_username: &str) -> Option<UserRow> {
    sqlx::query_as::<_, UserRow>(
        "SELECT id, clerk_user_id, github_username, github_id, email, name, avatar_url,
                reputation_score, trust_score, total_contributions, xp, level,
                created_at, updated_at
         FROM users WHERE github_username = $1",
    )
    .bind(github_username)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
}

pub async fn create(
    pool: &DbPool,
    clerk_user_id: &str,
    github_username: Option<&str>,
    email: Option<&str>,
    name: Option<&str>,
    avatar_url: Option<&str>,
) -> Result<UserRow, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "INSERT INTO users (clerk_user_id, github_username, email, name, avatar_url)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, clerk_user_id, github_username, github_id, email, name, avatar_url,
                   reputation_score, trust_score, total_contributions, xp, level,
                   created_at, updated_at",
    )
    .bind(clerk_user_id)
    .bind(github_username)
    .bind(email)
    .bind(name)
    .bind(avatar_url)
    .fetch_one(pool)
    .await
}

pub async fn upsert_from_clerk(
    pool: &DbPool,
    clerk_user_id: &str,
    email: Option<&str>,
    name: Option<&str>,
    avatar_url: Option<&str>,
) -> Result<UserRow, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "INSERT INTO users (clerk_user_id, email, name, avatar_url)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (clerk_user_id) DO UPDATE
            SET email = COALESCE(EXCLUDED.email, users.email),
                name = COALESCE(EXCLUDED.name, users.name),
                avatar_url = COALESCE(EXCLUDED.avatar_url, users.avatar_url),
                updated_at = CURRENT_TIMESTAMP
         RETURNING id, clerk_user_id, github_username, github_id, email, name, avatar_url,
                   reputation_score, trust_score, total_contributions, xp, level,
                   created_at, updated_at",
    )
    .bind(clerk_user_id)
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

pub async fn delete(pool: &DbPool, clerk_user_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE clerk_user_id = $1")
        .bind(clerk_user_id)
        .execute(pool)
        .await?;
    Ok(())
}
