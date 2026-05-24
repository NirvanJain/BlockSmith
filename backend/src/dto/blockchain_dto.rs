use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBlockDto {
    pub contributor: String,
    pub repository: String,
    pub contribution_type: String,
    pub contribution_link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockDto {
    pub index: u32,
    pub contributor: String,
    pub repository: String,
    pub contribution_type: String,
    pub previous_hash: String,
    pub hash: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainValidationDto {
    pub valid: bool,
    pub total_blocks: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainStatsDto {
    pub total_blocks: usize,
    pub total_contributors: usize,
    pub total_repositories: usize,
}