use bson::doc;
use futures_util::StreamExt;
use mongodb::options::FindOptions;

use super::db::DbPool;
use super::models::LeaderboardEntry;

pub async fn get_top(pool: &DbPool, limit: i64) -> Vec<LeaderboardEntry> {
    let col = pool.collection::<bson::Document>("users");
    let opts = FindOptions::builder()
        .sort(doc! { "reputation_score": -1 })
        .limit(Some(limit))
        .build();
    let mut cursor = col
        .find(doc! {}, opts)
        .await
        .unwrap();

    let mut entries = Vec::new();
    let mut rank = 1;
    while let Some(result) = cursor.next().await {
        if let Ok(doc) = result {
            entries.push(LeaderboardEntry {
                rank,
                user_id: uuid::Uuid::parse_str(
                    doc.get_str("user_id").unwrap_or("00000000-0000-0000-0000-000000000000")
                ).unwrap_or_default(),
                name: doc.get_str("name").ok().map(|s| s.to_string()),
                github_username: doc.get_str("github_username").ok().map(|s| s.to_string()),
                avatar_url: doc.get_str("avatar_url").ok().map(|s| s.to_string()),
                reputation_score: doc.get_i32("reputation_score").unwrap_or(0),
                xp: doc.get_i32("xp").unwrap_or(0),
                level: doc.get_i32("level").unwrap_or(1),
                total_contributions: doc.get_i32("total_contributions").unwrap_or(0),
            });
            rank += 1;
        }
    }
    entries
}
