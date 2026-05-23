use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(
            0,
            "Genesis Block".to_string(),
            "0".to_string(),
        );

        Self {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();

        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
        );

        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // Check if previous hash matches
            if current.previous_hash != previous.hash {
                return false;
            }

            // Recalculate current hash
            let recalculated_hash = Self::calculate_block_hash(
                current.index,
                &current.timestamp,
                &current.data,
                &current.previous_hash,
            );

            // Check if hash is tampered
            if current.hash != recalculated_hash {
                return false;
            }
        }

        true
    }

    fn calculate_block_hash(
        index: u32,
        timestamp: &str,
        data: &str,
        previous_hash: &str,
    ) -> String {
        use sha2::{Digest, Sha256};

        let input = format!(
            "{}{}{}{}",
            index,
            timestamp,
            data,
            previous_hash
        );

        let mut hasher = Sha256::new();

        hasher.update(input);

        hex::encode(hasher.finalize())
    }

    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("----------------------------");
            println!("Index: {}", block.index);
            println!("Timestamp: {}", block.timestamp);
            println!("Data: {}", block.data);
            println!("Previous Hash: {}", block.previous_hash);
            println!("Hash: {}", block.hash);
        }
    }
}