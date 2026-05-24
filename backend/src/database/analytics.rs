use sqlx::{
    Pool,
    Postgres,
    Row,
};

pub async fn total_users(
    pool: &Pool<Postgres>,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        "
        SELECT COUNT(*) as total
        FROM users
        ",
    )
    .fetch_one(pool)
    .await?;

    Ok(row.get("total"))
}

pub async fn total_blocks(
    pool: &Pool<Postgres>,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        "
        SELECT COUNT(*) as total
        FROM blocks
        ",
    )
    .fetch_one(pool)
    .await?;

    Ok(row.get("total"))
}

pub async fn total_contributions(
    pool: &Pool<Postgres>,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        "
        SELECT COUNT(*) as total
        FROM contributions
        ",
    )
    .fetch_one(pool)
    .await?;

    Ok(row.get("total"))
}