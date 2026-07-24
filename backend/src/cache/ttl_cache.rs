use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct TtlCache {
    store: Arc<
        Mutex<
            HashMap<
                String,
                (String, Instant),
            >,
        >,
    >,
}

impl Default for TtlCache {
    fn default() -> Self {
        Self::new()
    }
}

impl TtlCache {
    pub fn new() -> Self {
        Self {
            store: Arc::new(
                Mutex::new(HashMap::new()),
            ),
        }
    }

    pub fn set(
        &self,
        key: String,
        value: String,
        ttl_seconds: u64,
    ) {
        let expiration =
            Instant::now()
                + Duration::from_secs(
                    ttl_seconds,
                );

        let mut store =
            self.store.lock().unwrap();

        store.insert(
            key,
            (value, expiration),
        );
    }

    pub fn get(
        &self,
        key: &str,
    ) -> Option<String> {
        let mut store =
            self.store.lock().unwrap();

        if let Some((
            value,
            expiration,
        )) = store.get(key)
        {
            if Instant::now() < *expiration {
                return Some(value.clone());
            } else {
                store.remove(key);
            }
        }

        None
    }

    pub fn remove(
        &self,
        key: &str,
    ) {
        let mut store =
            self.store.lock().unwrap();

        store.remove(key);
    }

    pub fn clear(&self) {
        let mut store =
            self.store.lock().unwrap();

        store.clear();
    }
}