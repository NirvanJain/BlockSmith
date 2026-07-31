use mongodb::{Client, Database};
use std::env;

pub async fn create_pool() -> Result<Database, mongodb::error::Error> {
    let uri = env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let db_name = env::var("MONGODB_DB")
        .unwrap_or_else(|_| "blocksmith".to_string());

    let client = Client::with_uri_str(&uri).await?;
    let db = client.database(&db_name);

    Ok(db)
}