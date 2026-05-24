use sqlx::{
    Pool,
    Postgres,
    Transaction,
};

pub async fn begin_transaction(
    pool: &Pool<Postgres>,
) -> Result<
    Transaction<'_, Postgres>,
    sqlx::Error,
> {
    pool.begin().await
}

pub async fn commit_transaction(
    transaction: Transaction<
        '_,
        Postgres,
    >,
) -> Result<(), sqlx::Error> {
    transaction.commit().await
}

pub async fn rollback_transaction(
    transaction: Transaction<
        '_,
        Postgres,
    >,
) -> Result<(), sqlx::Error> {
    transaction.rollback().await
}