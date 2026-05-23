use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};

use std::sync::{Arc, Mutex};

use crate::block::Block;
use crate::blockchain::Blockchain;

// Shared blockchain type
pub type SharedBlockchain = Arc<Mutex<Blockchain>>;

#[derive(Deserialize)]
pub struct AddBlockRequest {
    pub data: String,
}

#[derive(Serialize)]
pub struct ValidationResponse {
    pub valid: bool,
}

// Create app routes
pub fn create_router(
    blockchain: SharedBlockchain,
) -> Router {
    Router::new()
        .route("/", get(home))
        .route("/blocks", get(get_blocks))
        .route("/add", post(add_block))
        .route("/validate", get(validate_chain))
        .with_state(blockchain)
}

// Home route
async fn home() -> &'static str {
    "Welcome to BlockSmith API"
}

// Get all blocks
async fn get_blocks(
    State(blockchain): State<SharedBlockchain>,
) -> Json<Vec<Block>> {
    let chain = blockchain.lock().unwrap();

    Json(chain.chain.clone())
}

// Add new block
async fn add_block(
    State(blockchain): State<SharedBlockchain>,
    Json(payload): Json<AddBlockRequest>,
) -> Json<String> {
    let mut chain = blockchain.lock().unwrap();

    chain.add_block(payload.data);

    Json("Block added successfully".to_string())
}

// Validate blockchain
async fn validate_chain(
    State(blockchain): State<SharedBlockchain>,
) -> Json<ValidationResponse> {
    let chain = blockchain.lock().unwrap();

    Json(ValidationResponse {
        valid: chain.is_valid(),
    })
}