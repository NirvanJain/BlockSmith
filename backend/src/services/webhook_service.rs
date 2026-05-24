use crate::github::webhooks::{
    PullRequestWebhook,
};

pub struct WebhookService;

impl WebhookService {
    pub fn process_pull_request(
        webhook:
            PullRequestWebhook,
    ) {
        println!(
            "Processing PR webhook from {}",
            webhook.sender.login
        );

        if webhook
            .pull_request
            .merged
            .unwrap_or(false)
        {
            println!(
                "Merged PR verified"
            );
        }
    }
}