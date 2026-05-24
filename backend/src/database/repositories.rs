use sqlx::{
    Pool,
    Postgres,
};

pub struct RepositoryManager {
    pub pool: Pool<Postgres>,
}

impl RepositoryManager {
    pub fn new(
        pool: Pool<Postgres>,
    ) -> Self {
        Self { pool }
    }

    pub fn pool(
        &self,
    ) -> &Pool<Postgres> {
        &self.pool
    }
}