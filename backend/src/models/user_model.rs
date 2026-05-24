use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserModel {
    pub id: i64,
    pub github_username: String,
    pub avatar_url: Option<String>,
    pub reputation_score: i32,
    pub total_contributions: i32,
    pub created_at: String,
}