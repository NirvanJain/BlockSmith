use bson::doc;
use uuid::Uuid;

use super::db::DbPool;
use super::models::UserRow;

const COLLECTION: &str = "users";

pub async fn find_by_id(pool: &DbPool, user_id: Uuid) -> Option<UserRow> {
    let col = pool.collection::<UserRow>(COLLECTION);
    col.find_one(doc! { "user_id": user_id.to_string() }, None)
        .await
        .ok()
        .flatten()
}

pub async fn find_by_github_id(pool: &DbPool, github_id: &str) -> Option<UserRow> {
    let col = pool.collection::<UserRow>(COLLECTION);
    col.find_one(doc! { "github_id": github_id }, None)
        .await
        .ok()
        .flatten()
}

pub async fn find_by_github_username(pool: &DbPool, github_username: &str) -> Option<UserRow> {
    let col = pool.collection::<UserRow>(COLLECTION);
    col.find_one(doc! { "github_username": github_username }, None)
        .await
        .ok()
        .flatten()
}

/// Upsert a user from GitHub OAuth.
pub async fn upsert_from_github(
    pool: &DbPool,
    github_id: &str,
    github_username: &str,
    email: Option<&str>,
    name: Option<&str>,
    avatar_url: Option<&str>,
    access_token: &str,
) -> Result<UserRow, mongodb::error::Error> {
    let col = pool.collection::<UserRow>(COLLECTION);
    let now = chrono::Utc::now();

    // Try to find existing user first
    if let Some(_existing) = col.find_one(doc! { "github_id": github_id }, None).await? {
        // Update existing
        let update = doc! {
            "$set": {
                "github_username": github_username,
                "github_access_token": access_token,
                "updated_at": bson::DateTime::from_millis(now.timestamp_millis()),
            }
        };
        col.update_one(doc! { "github_id": github_id }, update, None).await?;

        // Re-fetch to get updated doc
        return col.find_one(doc! { "github_id": github_id }, None).await?.ok_or_else(|| {
            mongodb::error::Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to re-fetch user after update",
            ))
        });
    }

    // Create new user
    let user_id = Uuid::new_v4();
    let user = UserRow {
        id: None,
        user_id,
        github_username: Some(github_username.to_string()),
        github_id: Some(github_id.to_string()),
        github_access_token: Some(access_token.to_string()),
        email: email.map(|s| s.to_string()),
        name: name.map(|s| s.to_string()),
        avatar_url: avatar_url.map(|s| s.to_string()),
        reputation_score: 0,
        trust_score: 0,
        total_contributions: 0,
        xp: 0,
        level: 1,
        created_at: now,
        updated_at: now,
    };

    col.insert_one(&user, None).await?;
    Ok(user)
}

pub async fn update(
    pool: &DbPool,
    user_id: Uuid,
    name: Option<&str>,
    email: Option<&str>,
    avatar_url: Option<&str>,
) -> Result<(), mongodb::error::Error> {
    let col = pool.collection::<UserRow>(COLLECTION);
    let mut update_doc = doc! {};

    if let Some(n) = name {
        update_doc.insert("name", n);
    }
    if let Some(e) = email {
        update_doc.insert("email", e);
    }
    if let Some(a) = avatar_url {
        update_doc.insert("avatar_url", a);
    }
    update_doc.insert("updated_at", bson::DateTime::from_millis(chrono::Utc::now().timestamp_millis()));

    col.update_one(
        doc! { "user_id": user_id.to_string() },
        doc! { "$set": update_doc },
        None,
    )
    .await?;
    Ok(())
}

pub async fn update_reputation(
    pool: &DbPool,
    user_id: Uuid,
    xp: i32,
    reputation_score: i32,
    level: i32,
) -> Result<(), mongodb::error::Error> {
    let col = pool.collection::<UserRow>(COLLECTION);
    col.update_one(
        doc! { "user_id": user_id.to_string() },
        doc! {
            "$set": {
                "xp": xp,
                "reputation_score": reputation_score,
                "level": level,
                "updated_at": bson::DateTime::from_millis(chrono::Utc::now().timestamp_millis()),
            }
        },
        None,
    )
    .await?;
    Ok(())
}

pub async fn delete_by_id(pool: &DbPool, user_id: Uuid) -> Result<(), mongodb::error::Error> {
    let col = pool.collection::<UserRow>(COLLECTION);
    col.delete_one(doc! { "user_id": user_id.to_string() }, None).await?;
    Ok(())
}

pub async fn delete_by_github_id(pool: &DbPool, github_id: &str) -> Result<(), mongodb::error::Error> {
    let col = pool.collection::<UserRow>(COLLECTION);
    col.delete_one(doc! { "github_id": github_id }, None).await?;
    Ok(())
}
