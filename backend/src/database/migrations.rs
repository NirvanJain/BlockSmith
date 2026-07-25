use bson::doc;

pub async fn run_migrations(pool: &mongodb::Database) -> Result<(), mongodb::error::Error> {
    // Create indexes for common queries
    let users_col = pool.collection::<bson::Document>("users");

    // Create unique index on github_id
    users_col
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "github_id": 1 })
                .options(
                    mongodb::options::IndexOptions::builder()
                        .unique(true)
                        .build(),
                )
                .build(),
            None,
        )
        .await?;

    // Create unique index on github_username
    users_col
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "github_username": 1 })
                .options(
                    mongodb::options::IndexOptions::builder()
                        .unique(true)
                        .sparse(true)
                        .build(),
                )
                .build(),
            None,
        )
        .await?;

    // Create unique index on user_id (UUID)
    users_col
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "user_id": 1 })
                .options(
                    mongodb::options::IndexOptions::builder()
                        .unique(true)
                        .build(),
                )
                .build(),
            None,
        )
        .await?;

    // Activities index
    let activities_col = pool.collection::<bson::Document>("activities");
    activities_col
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "created_at": -1 })
                .build(),
            None,
        )
        .await?;

    activities_col
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "user_id": 1 })
                .build(),
            None,
        )
        .await?;

    // Repositories unique index on github_id
    let repos_col = pool.collection::<bson::Document>("repositories");
    repos_col
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "github_id": 1 })
                .options(
                    mongodb::options::IndexOptions::builder()
                        .unique(true)
                        .build(),
                )
                .build(),
            None,
        )
        .await?;

    // Seed initial badges
    let badges_col = pool.collection::<bson::Document>("badges");
    let badges_count = badges_col.count_documents(doc! {}, None).await.unwrap_or(0);
    if badges_count == 0 {
        let _ = badges_col.insert_many(vec![
            doc! {
                "name": "First Merge",
                "description": "Merged your first pull request",
                "xp_required": 10,
            },
            doc! {
                "name": "Contributor",
                "description": "Earned 100 XP from open-source contributions",
                "xp_required": 100,
            },
            doc! {
                "name": "Maintainer",
                "description": "Earned 500 XP and created an issue/repository",
                "xp_required": 500,
            },
            doc! {
                "name": "Open Source Hero",
                "description": "Earned 2000 XP",
                "xp_required": 2000,
            },
        ], None).await;
    }

    Ok(())
}
