#[derive(Debug)]
pub struct CacheMonitor {
    pub total_hits: usize,
    pub total_misses: usize,
}

impl CacheMonitor {
    pub fn new() -> Self {
        Self {
            total_hits: 0,
            total_misses: 0,
        }
    }

    pub fn record_hit(
        &mut self,
    ) {
        self.total_hits += 1;
    }

    pub fn record_miss(
        &mut self,
    ) {
        self.total_misses += 1;
    }

    pub fn hit_ratio(
        &self,
    ) -> f64 {
        let total =
            self.total_hits
                + self.total_misses;

        if total == 0 {
            return 0.0;
        }

        self.total_hits as f64
            / total as f64
    }

    pub fn print_metrics(
        &self,
    ) {
        println!(
            "Cache Hits: {}",
            self.total_hits
        );

        println!(
            "Cache Misses: {}",
            self.total_misses
        );

        println!(
            "Hit Ratio: {:.2}",
            self.hit_ratio()
        );
    }
}