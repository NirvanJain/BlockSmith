use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionModel {
    pub id: i64,
    pub github_username: String,
    pub repository: String,
    pub contribution_type: String,
    pub contribution_link: String,
    pub verified: bool,
    pub created_at: String,
}