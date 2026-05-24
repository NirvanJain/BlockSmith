use crate::models::{
    metrics_model::MetricsModel,
};

pub struct MetricsService;

impl MetricsService {
    pub fn system_metrics(
        total_users: i64,
        total_blocks: i64,
        total_contributions: i64,
    ) -> MetricsModel {
        MetricsModel {
            total_users,
            total_blocks,
            total_contributions,
            cache_hit_ratio: 0.95,
            uptime_seconds: 10000,
        }
    }
}