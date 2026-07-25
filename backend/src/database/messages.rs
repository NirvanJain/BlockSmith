use bson::doc;
use futures_util::StreamExt;
use mongodb::options::FindOptions;
use uuid::Uuid;

use super::db::DbPool;
use super::models::{MessageRow, MessageResponse};

const COLLECTION: &str = "messages";

pub async fn get_messages(pool: &DbPool, conversation_id: Uuid) -> Vec<MessageResponse> {
    let col = pool.collection::<MessageRow>(COLLECTION);
    let users_col = pool.collection::<bson::Document>("users");

    let opts = FindOptions::builder()
        .sort(doc! { "created_at": 1 })
        .build();
    let mut cursor = col
        .find(doc! { "conversation_id": conversation_id.to_string() }, opts)
        .await
        .unwrap();

    let mut messages = Vec::new();
    while let Some(result) = cursor.next().await {
        if let Ok(msg) = result {
            let sender = users_col
                .find_one(doc! { "user_id": msg.sender_id.to_string() }, None)
                .await
                .ok()
                .flatten();

            messages.push(MessageResponse {
                id: msg.message_id,
                sender_id: msg.sender_id,
                sender_name: sender.as_ref().and_then(|s| s.get_str("name").ok()).map(|s| s.to_string()),
                sender_avatar: sender.as_ref().and_then(|s| s.get_str("avatar_url").ok()).map(|s| s.to_string()),
                content: msg.content,
                created_at: msg.created_at,
            });
        }
    }
    messages
}

pub async fn create(
    pool: &DbPool,
    conversation_id: Uuid,
    sender_id: Uuid,
    content: &str,
) -> Result<MessageRow, mongodb::error::Error> {
    let col = pool.collection::<MessageRow>(COLLECTION);
    let message_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let msg = MessageRow {
        id: None,
        message_id,
        conversation_id,
        sender_id,
        content: content.to_string(),
        created_at: now,
    };

    col.insert_one(&msg, None).await?;
    Ok(msg)
}
