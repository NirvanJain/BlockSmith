use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebsocketEventDto {
    pub event: String,
    pub message: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockCreatedEventDto {
    pub contributor: String,
    pub repository: String,
    pub block_hash: String,
}