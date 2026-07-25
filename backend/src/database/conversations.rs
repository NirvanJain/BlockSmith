use bson::doc;
use futures_util::StreamExt;
use uuid::Uuid;

use super::db::DbPool;
use super::models::{ConversationRow, ConversationWithParticipants, ParticipantInfo};

const COLLECTION: &str = "conversations";

pub async fn get_user_conversations(pool: &DbPool, _user_id: Uuid) -> Vec<ConversationWithParticipants> {
    let col = pool.collection::<ConversationRow>(COLLECTION);

    let cursor_result = col.find(doc! {}, None).await;
    let mut cursor = match cursor_result {
        Ok(c) => c,
        Err(_) => return vec![],
    };
    let mut convos = Vec::new();

    while let Some(result) = cursor.next().await {
        if let Ok(convo) = result {
            let participants = vec![];

            convos.push(ConversationWithParticipants {
                conversation: convo,
                participants,
            });
        }
    }
    convos
}

pub async fn find_existing_dm(pool: &DbPool, _user_a: Uuid, _user_b: Uuid) -> Option<ConversationRow> {
    let col = pool.collection::<ConversationRow>(COLLECTION);
    col.find_one(doc! { "is_group": false }, None).await.ok().flatten()
}

pub async fn create(pool: &DbPool) -> Result<ConversationRow, mongodb::error::Error> {
    let col = pool.collection::<ConversationRow>(COLLECTION);
    let convo_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let convo = ConversationRow {
        id: None,
        conversation_id: convo_id,
        is_group: false,
        name: None,
        created_at: now,
    };

    col.insert_one(&convo, None).await?;
    Ok(convo)
}

pub async fn add_participant(_pool: &DbPool, _conversation_id: Uuid, _user_id: Uuid) -> Result<(), mongodb::error::Error> {
    // TODO: implement conversation_participants collection
    Ok(())
}

pub async fn get_participants(_pool: &DbPool, _conversation_id: Uuid) -> Vec<ParticipantInfo> {
    vec![]
}
