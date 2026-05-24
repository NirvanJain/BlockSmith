use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContributionRequest {
    pub github_username: String,
    pub repository: String,
    pub contribution_type: String,
    pub contribution_link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyContributionRequest {
    pub contribution_link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubLoginRequest {
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBlockRequest {
    pub contributor: String,
    pub repository: String,
    pub contribution_type: String,
    pub contribution_link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchContributionRequest {
    pub query: String,
}