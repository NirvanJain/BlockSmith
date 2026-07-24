use crate::cache::{
    memory_cache::MemoryCache,
};

pub struct CacheService {
    pub cache: MemoryCache,
}

impl CacheService {
    pub fn new() -> Self {
        Self {
            cache:
                MemoryCache::new(),
        } 
    }

    pub fn set(
        &self,
        key: String,
        value: String,
    ) {
        self.cache.set(key, value);
    }

    pub fn get(
        &self,
        key: &str,
    ) -> Option<String> {
        self.cache.get(key)
    }

    pub fn delete(
        &self,
        key: &str,
    ) {
        self.cache.delete(key);
    }
}