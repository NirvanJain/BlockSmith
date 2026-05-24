use chrono::Utc;

use crate::{
    blockchain::{
        block::Block,
        hashing::generate_hash,
    },
    models::block_model::BlockModel,
};

pub fn create_block(
    index: u32,
    contributor: String,
    repository: String,
    contribution_type: String,
    contribution_link: String,
    previous_hash: String,
) -> Block {
    let timestamp =
        Utc::now().to_rfc3339();

    let hash = generate_hash(
        index,
        &timestamp,
        &contributor,
        &repository,
        &contribution_type,
        &contribution_link,
        &previous_hash,
    );

    Block {
        index,
        timestamp,
        contributor,
        repository,
        contribution_type,
        contribution_link,
        previous_hash,
        hash,
    }
}

pub fn convert_to_model(
    block: &Block,
    id: i64,
) -> BlockModel {
    BlockModel {
        id,
        block_index: block.index as i32,
        contributor:
            block.contributor.clone(),
        repository:
            block.repository.clone(),
        contribution_type:
            block.contribution_type.clone(),
        contribution_link:
            block.contribution_link.clone(),
        previous_hash:
            block.previous_hash.clone(),
        hash: block.hash.clone(),
        timestamp:
            block.timestamp.clone(),
    }
}