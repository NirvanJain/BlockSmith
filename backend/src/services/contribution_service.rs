use chrono::Utc;

use crate::models::contribution_model::ContributionModel;

pub fn create_contribution(
    id: i64,
    github_username: String,
    repository: String,
    contribution_type: String,
    contribution_link: String,
) -> ContributionModel {
    ContributionModel {
        id,
        github_username,
        repository,
        contribution_type,
        contribution_link,
        verified: true,
        created_at:
            Utc::now().to_rfc3339(),
    }
}