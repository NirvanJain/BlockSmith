use sqlx::Row;

use crate::database::db::DbPool;

pub async fn create_user(
    pool: &DbPool,
    github_username: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        INSERT INTO users (github_username)
        VALUES ($1)
        ",
    )
    .bind(github_username)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_user_by_username(
    pool: &DbPool,
    github_username: &str,
) -> Result<Option<i64>, sqlx::Error> {
    let row = sqlx::query(
        "
        SELECT id
        FROM users
        WHERE github_username = $1
        ",
    )
    .bind(github_username)
    .fetch_optional(pool)
    .await?;

    if let Some(user) = row {
        Ok(Some(user.get("id")))
    } else {
        Ok(None)
    }
}