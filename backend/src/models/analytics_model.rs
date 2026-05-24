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
pub struct AnalyticsModel {
    pub id: i64,
    pub metric_name: String,
    pub metric_value: i64,
    pub recorded_at: String,
}