use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardUserDto {
    pub username: String,
    pub reputation_score: i32,
    pub total_contributions: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryAnalyticsDto {
    pub repository: String,
    pub verified_contributions: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainAnalyticsDto {
    pub total_blocks: usize,
    pub total_users: usize,
    pub total_verified_contributions: usize,
}