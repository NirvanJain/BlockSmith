use bson::doc;
use uuid::Uuid;

use super::db::DbPool;

const COLLECTION: &str = "repositories";

pub async fn upsert(
    pool: &DbPool,
    github_id: i64,
    name: &str,
    owner: &str,
) -> Result<Uuid, mongodb::error::Error> {
    let col = pool.collection::<bson::Document>(COLLECTION);
    let now = chrono::Utc::now();

    let existing = col.find_one(doc! { "github_id": github_id }, None).await?;

    if let Some(doc) = existing {
        let repo_id_str = doc.get_str("repo_id").unwrap_or("");
        let repo_id = Uuid::parse_str(repo_id_str).unwrap_or_else(|_| Uuid::new_v4());

        col.update_one(
            doc! { "github_id": github_id },
            doc! { "$set": { "name": name, "owner": owner } },
            None,
        ).await?;

        return Ok(repo_id);
    }

    let repo_id = Uuid::new_v4();
    col.insert_one(doc! {
        "repo_id": repo_id.to_string(),
        "github_id": github_id,
        "name": name,
        "owner": owner,
        "created_at": bson::DateTime::from_millis(now.timestamp_millis()),
    }, None).await?;

    Ok(repo_id)
}

pub async fn increment_prs_merged(pool: &DbPool, user_id: Uuid) -> Result<(), mongodb::error::Error> {
    let col = pool.collection::<bson::Document>("contribution_stats");

    let existing = col.find_one(doc! { "user_id": user_id.to_string() }, None).await?;

    if existing.is_some() {
        col.update_one(
            doc! { "user_id": user_id.to_string() },
            doc! { "$inc": { "prs_merged": 1 } },
            None,
        ).await?;
    } else {
        col.insert_one(doc! {
            "user_id": user_id.to_string(),
            "prs_opened": 0,
            "prs_merged": 1,
            "issues_opened": 0,
            "commits_pushed": 0,
            "stars_given": 0,
        }, None).await?;
    }

    Ok(())
}

pub async fn record_reputation_change(
    pool: &DbPool,
    user_id: Uuid,
    amount: i32,
    reason: &str,
) -> Result<(), mongodb::error::Error> {
    let col = pool.collection::<bson::Document>("reputation_history");
    col.insert_one(doc! {
        "user_id": user_id.to_string(),
        "amount": amount,
        "reason": reason,
        "created_at": bson::DateTime::from_millis(chrono::Utc::now().timestamp_millis()),
    }, None).await?;
    Ok(())
}
