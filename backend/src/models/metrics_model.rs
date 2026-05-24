use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct MetricsModel {
    pub total_users: i64,
    pub total_blocks: i64,
    pub total_contributions: i64,
    pub cache_hit_ratio: f64,
    pub uptime_seconds: u64,
}