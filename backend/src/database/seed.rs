use sqlx::{
    Pool,
    Postgres,
};

pub async fn seed_demo_data(
    pool: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        INSERT INTO users
        (
            github_username,
            reputation_score,
            total_contributions
        )
        VALUES
        (
            'nirvanjain',
            120,
            15
        )
        ",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "
        INSERT INTO contributions
        (
            user_id,
            repository,
            contribution_type,
            contribution_link,
            verified
        )
        VALUES
        (
            1,
            'BlockSmith',
            'pull_request',
            'https://github.com/example/pull/1',
            TRUE
        )
        ",
    )
    .execute(pool)
    .await?;

    Ok(())
}