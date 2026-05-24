use crate::models::{
    analytics_model::AnalyticsModel,
    contribution_model::ContributionModel,
};

pub struct AnalyticsService;

impl AnalyticsService {
    pub fn total_contributions(
        contributions:
            &[ContributionModel],
    ) -> usize {
        contributions.len()
    }

    pub fn generate_metric(
        metric_name: String,
        metric_value: i64,
    ) -> AnalyticsModel {
        AnalyticsModel {
            id: 0,
            metric_name,
            metric_value,
            recorded_at:
                chrono::Utc::now()
                    .to_rfc3339(),
        }
    }
}