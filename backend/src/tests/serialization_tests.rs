use blocksmith::dto::notification_dto::NotificationDto;

#[test]
fn test_json_serialization() {
    let notification =
        NotificationDto {
            title:
                "Test".to_string(),

            message:
                "Serialization"
                    .to_string(),

            notification_type:
                "info".to_string(),

            timestamp:
                "2026".to_string(),
        };

    let json =
        serde_json::to_string(
            &notification,
        );

    assert!(json.is_ok());
}

#[test]
fn test_json_deserialization() {
    let json = r#"
    {
        "title":"Test",
        "message":"Hello",
        "notification_type":"info",
        "timestamp":"2026"
    }
    "#;

    let notification:
        Result<
            NotificationDto,
            _,
        > = serde_json::from_str(
        json,
    );

    assert!(notification.is_ok());
}