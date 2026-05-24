use crate::dto::notification_dto::{
    NotificationDto,
};

pub struct NotificationService;

impl NotificationService {
    pub fn send_notification(
        notification: NotificationDto,
    ) {
        println!(
            "Notification sent: {}",
            notification.title
        );
    }

    pub fn broadcast_notification(
        message: &str,
    ) {
        println!(
            "Broadcasting notification: {}",
            message
        );
    }
}