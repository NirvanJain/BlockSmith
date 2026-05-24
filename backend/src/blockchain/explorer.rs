use crate::blockchain::{
    block::Block,
    blockchain::Blockchain,
};

pub fn find_block_by_hash(
    blockchain: &Blockchain,
    hash: &str,
) -> Option<Block> {
    blockchain
        .chain
        .iter()
        .find(|block| block.hash == hash)
        .cloned()
}

pub fn find_blocks_by_contributor(
    blockchain: &Blockchain,
    contributor: &str,
) -> Vec<Block> {
    blockchain
        .chain
        .iter()
        .filter(|block| {
            block.contributor
                == contributor
        })
        .cloned()
        .collect()
}