use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: String,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u32,
        data: String,
        previous_hash: String,
    ) -> Self {
        let timestamp = Utc::now().to_rfc3339();

        let hash = Self::calculate_hash(
            index,
            &timestamp,
            &data,
            &previous_hash,
        );

        Self {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(
        index: u32,
        timestamp: &str,
        data: &str,
        previous_hash: &str,
    ) -> String {
        let input = format!(
            "{}{}{}{}",
            index,
            timestamp,
            data,
            previous_hash
        );

        let mut hasher = Sha256::new();

        hasher.update(input);

        let result = hasher.finalize();

        hex::encode(result)
    }
}