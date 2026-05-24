use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Serialize,
    Deserialize,
)]
pub struct CacheStatsDto {
    pub total_hits: usize,
    pub total_misses: usize,
    pub hit_ratio: f64,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
)]
pub struct CacheHealthDto {
    pub cache_available: bool,
    pub redis_connected: bool,
}