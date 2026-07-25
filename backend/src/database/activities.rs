use bson::doc;
use futures_util::StreamExt;
use mongodb::options::FindOptions;
use uuid::Uuid;

use super::db::DbPool;
use super::models::{ActivityRow, FeedItem};

const COLLECTION: &str = "activities";

#[allow(clippy::too_many_arguments)]
pub async fn create(
    pool: &DbPool,
    user_id: Uuid,
    activity_type: &str,
    repository_id: Option<Uuid>,
    title: &str,
    description: Option<&str>,
    link: Option<&str>,
    metadata: serde_json::Value,
    xp_earned: i32,
) -> Result<ActivityRow, mongodb::error::Error> {
    let col = pool.collection::<ActivityRow>(COLLECTION);
    let activity_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let activity = ActivityRow {
        id: None,
        activity_id,
        user_id,
        activity_type: activity_type.to_string(),
        repository_id,
        title: title.to_string(),
        description: description.map(|s| s.to_string()),
        link: link.map(|s| s.to_string()),
        metadata,
        xp_earned,
        created_at: now,
    };

    col.insert_one(&activity, None).await?;
    Ok(activity)
}

pub async fn get_feed(pool: &DbPool, limit: i64) -> Vec<FeedItem> {
    let col = pool.collection::<ActivityRow>(COLLECTION);
    let users_col = pool.collection::<super::models::UserRow>("users");

    let opts = FindOptions::builder()
        .sort(doc! { "created_at": -1 })
        .limit(Some(limit))
        .build();
    let mut cursor = col
        .find(doc! {}, opts)
        .await
        .unwrap();

    let mut items = Vec::new();
    while let Some(result) = cursor.next().await {
        if let Ok(activity) = result {
            let author = users_col
                .find_one(doc! { "user_id": activity.user_id.to_string() }, None)
                .await
                .ok()
                .flatten();

            items.push(FeedItem {
                id: activity.activity_id,
                author_name: author.as_ref().and_then(|a| a.name.clone()).unwrap_or_default(),
                author_username: author.as_ref().and_then(|a| a.github_username.clone()).unwrap_or_default(),
                author_avatar: author.as_ref().and_then(|a| a.avatar_url.clone()),
                activity_type: activity.activity_type,
                title: activity.title,
                description: activity.description,
                link: activity.link,
                repository: None,
                xp_earned: activity.xp_earned,
                created_at: activity.created_at,
            });
        }
    }
    items
}

pub async fn get_user_activities(pool: &DbPool, user_id: Uuid, limit: i64) -> Vec<ActivityRow> {
    let col = pool.collection::<ActivityRow>(COLLECTION);
    let opts = FindOptions::builder()
        .sort(doc! { "created_at": -1 })
        .limit(Some(limit))
        .build();
    let mut cursor = col
        .find(doc! { "user_id": user_id.to_string() }, opts)
        .await
        .unwrap();

    let mut items = Vec::new();
    while let Some(result) = cursor.next().await {
        if let Ok(item) = result {
            items.push(item);
        }
    }
    items
}
