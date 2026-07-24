use uuid::Uuid;

use super::db::DbPool;
use super::models::{ActivityRow, FeedItem};

pub async fn create(
    pool: &DbPool,
    user_id: Uuid,
    activity_type: &str,
    repository_id: Option<Uuid>,
    title: &str,
    description: Option<&str>,
    link: Option<&str>,
    metadata: serde_json::Value,
    xp_earned: i32,
) -> Result<ActivityRow, sqlx::Error> {
    sqlx::query_as::<_, ActivityRow>(
        "INSERT INTO activities (user_id, activity_type, repository_id, title, description, link, metadata, xp_earned)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING id, user_id, activity_type, repository_id, title, description, link, metadata, xp_earned, created_at",
    )
    .bind(user_id)
    .bind(activity_type)
    .bind(repository_id)
    .bind(title)
    .bind(description)
    .bind(link)
    .bind(metadata)
    .bind(xp_earned)
    .fetch_one(pool)
    .await
}

pub async fn get_feed(pool: &DbPool, limit: i64) -> Vec<FeedItem> {
    let rows = sqlx::query(
        "SELECT a.id, a.activity_type, a.title, a.description, a.link, a.xp_earned, a.created_at,
                u.name as author_name, u.github_username as author_username, u.avatar_url as author_avatar,
                r.name as repository_name
         FROM activities a
         JOIN users u ON a.user_id = u.id
         LEFT JOIN repositories r ON a.repository_id = r.id
         ORDER BY a.created_at DESC
         LIMIT $1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter()
        .map(|row| FeedItem {
            id: row.get("id"),
            author_name: row.get("author_name"),
            author_username: row.get("author_username"),
            author_avatar: row.get("author_avatar"),
            activity_type: row.get("activity_type"),
            title: row.get("title"),
            description: row.get("description"),
            link: row.get("link"),
            repository: row.get("repository_name"),
            xp_earned: row.get("xp_earned"),
            created_at: row.get("created_at"),
        })
        .collect()
}

pub async fn get_user_activities(pool: &DbPool, user_id: Uuid, limit: i64) -> Vec<ActivityRow> {
    sqlx::query_as::<_, ActivityRow>(
        "SELECT id, user_id, activity_type, repository_id, title, description, link, metadata, xp_earned, created_at
         FROM activities WHERE user_id = $1
         ORDER BY created_at DESC LIMIT $2",
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .unwrap_or_default()
}
