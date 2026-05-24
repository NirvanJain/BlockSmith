use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockModel {
    pub id: i64,
    pub block_index: i32,
    pub contributor: String,
    pub repository: String,
    pub contribution_type: String,
    pub contribution_link: String,
    pub previous_hash: String,
    pub hash: String,
    pub timestamp: String,
}