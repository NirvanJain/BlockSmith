use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct PresenceManager {
    online_users:
        Arc<Mutex<HashSet<String>>>,
}

impl Default for PresenceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PresenceManager {
    pub fn new() -> Self {
        Self {
            online_users: Arc::new(
                Mutex::new(HashSet::new()),
            ),
        }
    }

    pub fn user_connected(
        &self,
        username: String,
    ) {
        let mut users =
            self.online_users
                .lock()
                .unwrap();

        users.insert(username);
    }

    pub fn user_disconnected(
        &self,
        username: &str,
    ) {
        let mut users =
            self.online_users
                .lock()
                .unwrap();

        users.remove(username);
    }

    pub fn online_count(
        &self,
    ) -> usize {
        let users =
            self.online_users
                .lock()
                .unwrap();

        users.len()
    }
}