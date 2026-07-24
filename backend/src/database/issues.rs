use sqlx::Row;

use super::db::DbPool;
use super::models::DiscoveryItem;

pub async fn get_discovery(pool: &DbPool, limit: i64) -> Vec<DiscoveryItem> {
    let rows = sqlx::query(
        "SELECT i.id as issue_id, i.title, i.body, i.state, i.labels, i.creator_username,
                i.ai_complexity_score, i.ai_match_score, i.ai_analysis, i.created_at,
                r.name as repository_name, r.owner as repository_owner
         FROM issues i
         JOIN repositories r ON i.repository_id = r.id
         WHERE i.state = 'open'
         ORDER BY i.created_at DESC
         LIMIT $1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter()
        .map(|row| DiscoveryItem {
            issue_id: row.get("issue_id"),
            title: row.get("title"),
            body: row.get("body"),
            state: row.get("state"),
            labels: row.get("labels"),
            creator_username: row.get("creator_username"),
            ai_complexity_score: row.get("ai_complexity_score"),
            ai_match_score: row.get("ai_match_score"),
            ai_analysis: row.get("ai_analysis"),
            repository_name: row.get("repository_name"),
            repository_owner: row.get("repository_owner"),
            created_at: row.get("created_at"),
        })
        .collect()
}
