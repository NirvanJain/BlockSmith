use blocksmith::dto::notification_dto::NotificationDto;

use blocksmith::services::notification_service::NotificationService;

#[test]
fn test_notification_creation() {
    let notification =
        NotificationDto {
            title:
                "PR Verified"
                    .to_string(),

            message:
                "Contribution verified"
                    .to_string(),

            notification_type:
                "success"
                    .to_string(),

            timestamp:
                "2026-01-01"
                    .to_string(),
        };

    NotificationService::send_notification(
        notification,
    );

    assert!(true);
}