use uuid::Uuid;

use super::db::DbPool;
use super::models::{MessageRow, MessageResponse};

pub async fn get_messages(pool: &DbPool, conversation_id: Uuid) -> Vec<MessageResponse> {
    let rows = sqlx::query(
        "SELECT m.id, m.sender_id, m.content, m.created_at,
                u.name as sender_name, u.avatar_url as sender_avatar
         FROM messages m
         JOIN users u ON m.sender_id = u.id
         WHERE m.conversation_id = $1
         ORDER BY m.created_at ASC",
    )
    .bind(conversation_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter()
        .map(|row| MessageResponse {
            id: row.get("id"),
            sender_id: row.get("sender_id"),
            sender_name: row.get("sender_name"),
            sender_avatar: row.get("sender_avatar"),
            content: row.get("content"),
            created_at: row.get("created_at"),
        })
        .collect()
}

pub async fn create(
    pool: &DbPool,
    conversation_id: Uuid,
    sender_id: Uuid,
    content: &str,
) -> Result<MessageRow, sqlx::Error> {
    sqlx::query_as::<_, MessageRow>(
        "INSERT INTO messages (conversation_id, sender_id, content)
         VALUES ($1, $2, $3)
         RETURNING id, conversation_id, sender_id, content, created_at",
    )
    .bind(conversation_id)
    .bind(sender_id)
    .bind(content)
    .fetch_one(pool)
    .await
}
