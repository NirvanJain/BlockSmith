use blocksmith::services::reputation_service::{
    calculate_reputation,
};

#[test]
fn test_reputation_score() {
    let score =
        calculate_reputation(
            "pull_request",
        );

    assert_eq!(score, 10);
}