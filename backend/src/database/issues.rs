use bson::doc;
use futures_util::StreamExt;
use mongodb::options::FindOptions;

use super::db::DbPool;
use super::models::DiscoveryItem;

pub async fn get_discovery(pool: &DbPool, limit: i64) -> Vec<DiscoveryItem> {
    let issues_col = pool.collection::<bson::Document>("issues");

    let opts = FindOptions::builder()
        .sort(doc! { "created_at": -1 })
        .limit(Some(limit))
        .build();
    let mut cursor = issues_col
        .find(doc! {}, opts)
        .await
        .unwrap();

    let mut items = Vec::new();
    while let Some(result) = cursor.next().await {
        if let Ok(issue) = result {
            let repo_name = issue.get_str("repository_name").unwrap_or("").to_string();
            let repo_owner = issue.get_str("repository_owner").unwrap_or("").to_string();

            items.push(DiscoveryItem {
                issue_id: uuid::Uuid::parse_str(
                    issue.get_str("issue_id").unwrap_or("00000000-0000-0000-0000-000000000000")
                ).unwrap_or_default(),
                title: issue.get_str("title").unwrap_or("").to_string(),
                body: issue.get_str("body").ok().map(|s| s.to_string()),
                state: issue.get_str("state").unwrap_or("open").to_string(),
                labels: issue.get_array("labels")
                    .unwrap_or(&vec![])
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect(),
                creator_username: issue.get_str("creator_username").unwrap_or("").to_string(),
                ai_complexity_score: issue.get_i32("ai_complexity_score").ok(),
                ai_match_score: issue.get_i32("ai_match_score").ok(),
                ai_analysis: issue.get_str("ai_analysis").ok().map(|s| s.to_string()),
                repository_name: repo_name,
                repository_owner: repo_owner,
                created_at: chrono::Utc::now(),
            });
        }
    }
    items
}
