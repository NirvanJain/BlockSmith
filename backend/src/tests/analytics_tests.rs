use blocksmith::services::reputation_service::{
    calculate_total_score,
};

#[test]
fn test_analytics_score() {
    let contributions = vec![
        "pull_request".to_string(),
        "issue".to_string(),
        "commit".to_string(),
    ];

    let score =
        calculate_total_score(
            contributions,
        );

    assert_eq!(score, 18);
}