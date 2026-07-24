use sqlx::Row;

use super::db::DbPool;
use super::models::LeaderboardEntry;

pub async fn get_top(pool: &DbPool, limit: i64) -> Vec<LeaderboardEntry> {
    let rows = sqlx::query(
        "SELECT u.id as user_id, u.name, u.github_username, u.avatar_url,
                u.reputation_score, u.xp, u.level, u.total_contributions
         FROM users u
         ORDER BY u.reputation_score DESC
         LIMIT $1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter()
        .enumerate()
        .map(|(i, row)| LeaderboardEntry {
            rank: (i + 1) as i32,
            user_id: row.get("user_id"),
            name: row.get("name"),
            github_username: row.get("github_username"),
            avatar_url: row.get("avatar_url"),
            reputation_score: row.get("reputation_score"),
            xp: row.get("xp"),
            level: row.get("level"),
            total_contributions: row.get("total_contributions"),
        })
        .collect()
}
