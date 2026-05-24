use chrono::Utc;

use crate::blockchain::{
    block::Block,
    hashing::generate_hash,
};

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now().to_rfc3339(),
            contributor: "BlockSmith".to_string(),
            repository: "Genesis".to_string(),
            contribution_type: "Genesis Block"
                .to_string(),
            contribution_link: "None".to_string(),
            previous_hash: "0".to_string(),
            hash: "genesis_hash".to_string(),
        };

        Self {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(
        &mut self,
        contributor: String,
        repository: String,
        contribution_type: String,
        contribution_link: String,
    ) {
        let previous_block =
            self.chain.last().unwrap();

        let index = previous_block.index + 1;

        let timestamp =
            Utc::now().to_rfc3339();

        let previous_hash =
            previous_block.hash.clone();

        let hash = generate_hash(
            index,
            &timestamp,
            &contributor,
            &repository,
            &contribution_type,
            &contribution_link,
            &previous_hash,
        );

        let block = Block {
            index,
            timestamp,
            contributor,
            repository,
            contribution_type,
            contribution_link,
            previous_hash,
            hash,
        };

        self.chain.push(block);
    }

    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
    }
}