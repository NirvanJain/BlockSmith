use blocksmith::models::leaderboard_model::LeaderboardModel;

use blocksmith::services::leaderboard_service::LeaderboardService;

#[test]
fn test_leaderboard_sorting() {
    let entries = vec![
        LeaderboardModel {
            id: 1,
            user_id: 1,
            reputation_score: 50,
            verified_contributions: 5,
            rank_position: 2,
            updated_at:
                "2026".to_string(),
        },
        LeaderboardModel {
            id: 2,
            user_id: 2,
            reputation_score: 100,
            verified_contributions: 10,
            rank_position: 1,
            updated_at:
                "2026".to_string(),
        },
    ];

    let sorted =
        LeaderboardService::sort_leaderboard(
            entries,
        );

    assert_eq!(
        sorted[0]
            .reputation_score,
        100
    );
}