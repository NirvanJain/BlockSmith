use uuid::Uuid;

use super::db::DbPool;
use super::models::ProfileRow;

pub async fn get(pool: &DbPool, user_id: Uuid) -> Option<ProfileRow> {
    sqlx::query_as::<_, ProfileRow>(
        "SELECT user_id, bio, location, website, twitter, linkedin, company,
                skills, interests, updated_at
         FROM profiles WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
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
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO profiles (user_id, bio, location, website, company, skills, interests)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         ON CONFLICT (user_id) DO UPDATE
            SET bio = COALESCE(EXCLUDED.bio, profiles.bio),
                location = COALESCE(EXCLUDED.location, profiles.location),
                website = COALESCE(EXCLUDED.website, profiles.website),
                company = COALESCE(EXCLUDED.company, profiles.company),
                skills = COALESCE(EXCLUDED.skills, profiles.skills),
                interests = COALESCE(EXCLUDED.interests, profiles.interests),
                updated_at = CURRENT_TIMESTAMP",
    )
    .bind(user_id)
    .bind(bio)
    .bind(location)
    .bind(website)
    .bind(company)
    .bind(skills)
    .bind(interests)
    .execute(pool)
    .await?;
    Ok(())
}
