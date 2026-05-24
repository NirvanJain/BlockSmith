use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContributionResponse {
    pub id: i64,
    pub github_username: String,
    pub repository: String,
    pub contribution_type: String,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResponse {
    pub index: u32,
    pub contributor: String,
    pub repository: String,
    pub contribution_type: String,
    pub hash: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResponse {
    pub valid: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubUserResponse {
    pub login: String,
    pub avatar_url: String,
    pub profile_url: String,
}