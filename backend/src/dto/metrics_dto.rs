use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Serialize,
    Deserialize,
)]
pub struct SystemMetricsDto {
    pub total_users: usize,
    pub total_blocks: usize,
    pub total_contributions:
        usize,
    pub cache_hit_ratio: f64,
    pub uptime_seconds: u64,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
)]
pub struct ApiMetricsDto {
    pub requests_per_minute: usize,
    pub average_response_time_ms:
        f64,
}