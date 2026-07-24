use sqlx::Row;
use uuid::Uuid;

use super::db::DbPool;
use super::models::{ConversationRow, ConversationWithParticipants, ParticipantInfo};

pub async fn get_user_conversations(pool: &DbPool, user_id: Uuid) -> Vec<ConversationWithParticipants> {
    let convos = sqlx::query_as::<_, ConversationRow>(
        "SELECT c.id, c.is_group, c.name, c.created_at
         FROM conversations c
         JOIN conversation_participants cp ON c.id = cp.conversation_id
         WHERE cp.user_id = $1
         ORDER BY c.created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    let mut result = Vec::new();
    for convo in convos {
        let participants = get_participants(pool, convo.id).await;
        result.push(ConversationWithParticipants {
            conversation: convo,
            participants,
        });
    }
    result
}

pub async fn get_participants(pool: &DbPool, conversation_id: Uuid) -> Vec<ParticipantInfo> {
    let rows = sqlx::query(
        "SELECT u.id as user_id, u.name, u.github_username, u.avatar_url
         FROM users u
         JOIN conversation_participants cp ON u.id = cp.user_id
         WHERE cp.conversation_id = $1",
    )
    .bind(conversation_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter()
        .map(|row| ParticipantInfo {
            user_id: row.get("user_id"),
            name: row.get("name"),
            github_username: row.get("github_username"),
            avatar_url: row.get("avatar_url"),
        })
        .collect()
}

pub async fn find_existing_dm(pool: &DbPool, user_a: Uuid, user_b: Uuid) -> Option<ConversationRow> {
    sqlx::query_as::<_, ConversationRow>(
        "SELECT c.id, c.is_group, c.name, c.created_at
         FROM conversations c
         JOIN conversation_participants cp1 ON c.id = cp1.conversation_id AND cp1.user_id = $1
         JOIN conversation_participants cp2 ON c.id = cp2.conversation_id AND cp2.user_id = $2
         WHERE c.is_group = FALSE",
    )
    .bind(user_a)
    .bind(user_b)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
}

pub async fn create(pool: &DbPool) -> Result<ConversationRow, sqlx::Error> {
    sqlx::query_as::<_, ConversationRow>(
        "INSERT INTO conversations (is_group) VALUES (FALSE)
         RETURNING id, is_group, name, created_at",
    )
    .fetch_one(pool)
    .await
}

pub async fn add_participant(
    pool: &DbPool,
    conversation_id: Uuid,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO conversation_participants (conversation_id, user_id)
         VALUES ($1, $2)
         ON CONFLICT DO NOTHING",
    )
    .bind(conversation_id)
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}
