use blocksmith::services::metrics_service::MetricsService;

#[test]
fn test_metrics_generation() {
    let metrics =
        MetricsService::system_metrics(
            100,
            50,
            200,
        );

    assert_eq!(
        metrics.total_users,
        100
    );

    assert_eq!(
        metrics.total_blocks,
        50
    );
}