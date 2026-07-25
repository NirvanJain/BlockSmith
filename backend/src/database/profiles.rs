use bson::doc;
use uuid::Uuid;

use super::db::DbPool;
use super::models::ProfileRow;

const COLLECTION: &str = "profiles";

pub async fn get(pool: &DbPool, user_id: Uuid) -> Option<ProfileRow> {
    let col = pool.collection::<ProfileRow>(COLLECTION);
    col.find_one(doc! { "user_id": user_id.to_string() }, None)
        .await
        .ok()
        .flatten()
}

pub async fn upsert(
    pool: &DbPool,
    user_id: Uuid,
    bio: Option<&str>,
    location: Option<&str>,
    website: Option<&str>,
    company: Option<&str>,
    skills: &[String],
    interests: &[String],
) -> Result<(), mongodb::error::Error> {
    let col = pool.collection::<ProfileRow>(COLLECTION);
    let now = chrono::Utc::now();

    let existing = col.find_one(doc! { "user_id": user_id.to_string() }, None).await?;

    if existing.is_some() {
        let mut update = doc! {};
        if let Some(b) = bio { update.insert("bio", b); }
        if let Some(l) = location { update.insert("location", l); }
        if let Some(w) = website { update.insert("website", w); }
        if let Some(c) = company { update.insert("company", c); }
        if !skills.is_empty() { update.insert("skills", bson::to_bson(skills).unwrap_or_default()); }
        if !interests.is_empty() { update.insert("interests", bson::to_bson(interests).unwrap_or_default()); }
        update.insert("updated_at", bson::DateTime::from_millis(now.timestamp_millis()));

        col.update_one(
            doc! { "user_id": user_id.to_string() },
            doc! { "$set": update },
            None,
        ).await?;
    } else {
        let profile = ProfileRow {
            id: None,
            user_id,
            bio: bio.map(|s| s.to_string()),
            location: location.map(|s| s.to_string()),
            website: website.map(|s| s.to_string()),
            twitter: None,
            linkedin: None,
            company: company.map(|s| s.to_string()),
            skills: skills.to_vec(),
            interests: interests.to_vec(),
            updated_at: now,
        };
        col.insert_one(&profile, None).await?;
    }

    Ok(())
}
