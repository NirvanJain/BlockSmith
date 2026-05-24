use sqlx::Row;

use crate::database::db::DbPool;

pub async fn create_contribution(
    pool: &DbPool,
    user_id: i64,
    repo_name: &str,
    contribution_type: &str,
    contribution_link: &str,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        "
        INSERT INTO contributions
        (
            user_id,
            repo_name,
            contribution_type,
            contribution_link,
            verified
        )
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        ",
    )
    .bind(user_id)
    .bind(repo_name)
    .bind(contribution_type)
    .bind(contribution_link)
    .bind(true)
    .fetch_one(pool)
    .await?;

    Ok(row.get("id"))
}

pub async fn get_all_contributions(
    pool: &DbPool,
) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query(
        "
        SELECT contribution_link
        FROM contributions
        ",
    )
    .fetch_all(pool)
    .await?;

    let contributions = rows
        .into_iter()
        .map(|row| row.get("contribution_link"))
        .collect();

    Ok(contributions)
}