use uuid::Uuid;

use super::db::DbPool;
use super::models::BadgeRow;

pub async fn get_user_badges(pool: &DbPool, user_id: Uuid) -> Vec<BadgeRow> {
    sqlx::query_as::<_, BadgeRow>(
        "SELECT b.id, b.name, b.description, b.icon_url, b.xp_required
         FROM badges b
         JOIN user_badges ub ON b.id = ub.badge_id
         WHERE ub.user_id = $1
         ORDER BY ub.awarded_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default()
}

pub async fn award_badge(pool: &DbPool, user_id: Uuid, badge_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO user_badges (user_id, badge_id)
         VALUES ($1, $2)
         ON CONFLICT (user_id, badge_id) DO NOTHING",
    )
    .bind(user_id)
    .bind(badge_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_all(pool: &DbPool) -> Vec<BadgeRow> {
    sqlx::query_as::<_, BadgeRow>(
        "SELECT id, name, description, icon_url, xp_required
         FROM badges ORDER BY xp_required ASC",
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default()
}
