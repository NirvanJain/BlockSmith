use sqlx::{Pool, Postgres};

pub async fn run_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::migrate::MigrateError> {
    println!("Running OpenHub database migrations...");
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    println!("All migrations completed successfully");
    Ok(())
}