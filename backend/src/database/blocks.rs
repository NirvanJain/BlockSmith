use sqlx::Row;

use crate::database::db::DbPool;

pub async fn create_block(
    pool: &DbPool,
    block_index: i32,
    contribution_id: i64,
    previous_hash: &str,
    hash: &str,
    timestamp: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        INSERT INTO blocks
        (
            block_index,
            contribution_id,
            previous_hash,
            hash,
            timestamp
        )
        VALUES ($1, $2, $3, $4, $5)
        ",
    )
    .bind(block_index)
    .bind(contribution_id)
    .bind(previous_hash)
    .bind(hash)
    .bind(timestamp)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_latest_block_hash(
    pool: &DbPool,
) -> Result<Option<String>, sqlx::Error> {
    let row = sqlx::query(
        "
        SELECT hash
        FROM blocks
        ORDER BY id DESC
        LIMIT 1
        ",
    )
    .fetch_optional(pool)
    .await?; 

    if let Some(block) = row {
        Ok(Some(block.get("hash")))
    } else {
        Ok(None)
    }
}