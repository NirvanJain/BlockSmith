use blocksmith::services::repository_service::RepositoryService;

use blocksmith::models::repository_model::RepositoryModel;

#[test]
fn test_repository_summary() {
    let repository = RepositoryModel {
        id: 1,
        github_repo_id: 100,
        owner: "nirvanjain".to_string(),
        name: "BlockSmith".to_string(),
        full_name:
            "nirvanjain/BlockSmith"
                .to_string(),
        description: Some(
            "Blockchain contribution verifier"
                .to_string(),
        ),
        stars: 120,
        forks: 25,
        language: Some(
            "Rust".to_string(),
        ),
        created_at:
            "2026-01-01".to_string(),
    };

    let summary =
        RepositoryService::repository_summary(
            &repository,
        );

    assert!(
        summary.contains(
            "BlockSmith"
        )
    );
}

#[test]
fn test_popular_repository() {
    let repository = RepositoryModel {
        id: 1,
        github_repo_id: 100,
        owner: "nirvanjain".to_string(),
        name: "BlockSmith".to_string(),
        full_name:
            "nirvanjain/BlockSmith"
                .to_string(),
        description: None,
        stars: 500,
        forks: 40,
        language: Some(
            "Rust".to_string(),
        ),
        created_at:
            "2026-01-01".to_string(),
    };

    assert!(
        RepositoryService::is_popular(
            &repository
        )
    );
}