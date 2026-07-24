use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct MemoryCache {
    store: Arc<
        Mutex<HashMap<String, String>>
    >,
}

impl Default for MemoryCache {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryCache {
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
    ) {
        let mut store =
            self.store.lock().unwrap();

        store.insert(key, value);
    }

    pub fn get(
        &self,
        key: &str,
    ) -> Option<String> {
        let store =
            self.store.lock().unwrap();

        store.get(key).cloned()
    }

    pub fn delete(
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