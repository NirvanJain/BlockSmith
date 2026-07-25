use uuid::Uuid;

use super::db::DbPool;

/// Upsert a repository by its GitHub ID. Returns the repository UUID.
pub async fn upsert(
    pool: &DbPool,
    github_id: i64,
    name: &str,
    owner: &str,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO repositories (github_id, name, owner)
         VALUES ($1, $2, $3)
         ON CONFLICT (github_id) DO UPDATE SET name = EXCLUDED.name, owner = EXCLUDED.owner
         RETURNING id",
    )
    .bind(github_id)
    .bind(name)
    .bind(owner)
    .fetch_one(pool)
    .await
}

/// Increment the prs_merged counter for a user. Creates the row if it doesn't exist.
pub async fn increment_prs_merged(pool: &DbPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO contribution_stats (user_id, prs_merged)
         VALUES ($1, 1)
         ON CONFLICT (user_id) DO UPDATE
         SET prs_merged = contribution_stats.prs_merged + 1",
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Record a reputation change in the history table.
pub async fn record_reputation_change(
    pool: &DbPool,
    user_id: Uuid,
    amount: i32,
    reason: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO reputation_history (user_id, amount, reason)
         VALUES ($1, $2, $3)",
    )
    .bind(user_id)
    .bind(amount)
    .bind(reason)
    .execute(pool)
    .await?;
    Ok(())
}
