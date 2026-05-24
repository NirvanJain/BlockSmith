use blocksmith::services::queue_service::QueueService;

#[test]
fn test_queue_operations() {
    let queue =
        QueueService::new();

    queue.enqueue(
        "verify_pr".to_string(),
    );

    assert_eq!(queue.size(), 1);

    let job =
        queue.dequeue();

    assert_eq!(
        job.unwrap(),
        "verify_pr"
    );
}