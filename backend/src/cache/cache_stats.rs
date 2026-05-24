#[derive(Debug, Default)]
pub struct CacheStats {
    pub hits: usize,
    pub misses: usize,
}

impl CacheStats {
    pub fn new() -> Self {
        Self {
            hits: 0,
            misses: 0,
        }
    }

    pub fn hit(
        &mut self,
    ) {
        self.hits += 1;
    }

    pub fn miss(
        &mut self,
    ) {
        self.misses += 1;
    }

    pub fn print_stats(
        &self,
    ) {
        println!(
            "Cache Hits: {}",
            self.hits
        );

        println!(
            "Cache Misses: {}",
            self.misses
        );
    }
}