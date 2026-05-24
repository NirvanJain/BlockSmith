use blocksmith::github::webhooks::{
    PullRequestWebhook,
    PullRequestInfo,
    WebhookRepository,
    WebhookSender,
};

#[test]
fn test_webhook_payload() {
    let webhook =
        PullRequestWebhook {
            action:
                "closed"
                    .to_string(),

            repository:
                WebhookRepository {
                    full_name:
                        "BlockSmith"
                            .to_string(),
                },

            sender: WebhookSender {
                login:
                    "nirvanjain"
                        .to_string(),
            },

            pull_request:
                PullRequestInfo {
                    html_url:
                        "https://github.com"
                            .to_string(),

                    merged:
                        Some(true),
                },
        };

    assert_eq!(
        webhook.action,
        "closed"
    );
}