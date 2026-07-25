use bson::doc;
use futures_util::StreamExt;
use uuid::Uuid;

use super::db::DbPool;
use super::models::BadgeRow;

const COLLECTION: &str = "badges";

pub async fn get_user_badges(_pool: &DbPool, _user_id: Uuid) -> Vec<BadgeRow> {
    vec![]
}

pub async fn award_badge(_pool: &DbPool, _user_id: Uuid, _badge_id: Uuid) -> Result<(), mongodb::error::Error> {
    // TODO: implement user_badges collection
    Ok(())
}

pub async fn get_all(pool: &DbPool) -> Vec<BadgeRow> {
    let col = pool.collection::<BadgeRow>(COLLECTION);
    let cursor_result = col.find(doc! {}, None).await;
    let mut cursor = match cursor_result {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let mut badges = Vec::new();
    while let Some(result) = cursor.next().await {
        if let Ok(badge) = result {
            badges.push(badge);
        }
    }
    badges
}
