use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct SubscriptionManager {
    subscriptions: Arc<
        Mutex<
            HashMap<
                String,
                Vec<String>,
            >,
        >,
    >,
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            subscriptions:
                Arc::new(
                    Mutex::new(
                        HashMap::new(),
                    ),
                ),
        }
    }

    pub fn subscribe(
        &self,
        username: String,
        topic: String,
    ) {
        let mut subscriptions =
            self.subscriptions
                .lock()
                .unwrap();

        subscriptions
            .entry(username)
            .or_default()
            .push(topic);
    }

    pub fn get_subscriptions(
        &self,
        username: &str,
    ) -> Vec<String> {
        let subscriptions =
            self.subscriptions
                .lock()
                .unwrap();

        subscriptions
            .get(username)
            .cloned()
            .unwrap_or_default()
    }
}