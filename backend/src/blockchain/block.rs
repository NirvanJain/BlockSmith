use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u32,
    pub timestamp: String,
    pub contributor: String,
    pub repository: String,
    pub contribution_type: String,
    pub contribution_link: String,
    pub previous_hash: String,
    pub hash: String,
}