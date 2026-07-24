use crate::cache::memory_cache::MemoryCache;

#[derive(Clone)]
pub struct CacheManager {
    pub github_cache: MemoryCache,
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            github_cache:
                MemoryCache::new(),
        }
    }

    pub fn clear_all(&self) {
        self.github_cache.clear();
    }
}