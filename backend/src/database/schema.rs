use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSchema {
    pub id: i64,
    pub github_username: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContributionSchema {
    pub id: i64,
    pub user_id: i64,
    pub repo_name: String,
    pub contribution_type: String,
    pub contribution_link: String,
    pub verified: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockSchema {
    pub id: i64,
    pub block_index: i32,
    pub contribution_id: i64,
    pub previous_hash: String,
    pub hash: String,
    pub timestamp: String,
}