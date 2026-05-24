use std::collections::HashSet;

use crate::blockchain::blockchain::Blockchain;

pub struct ChainStats {
    pub total_blocks: usize,
    pub total_contributors: usize,
}

pub fn calculate_chain_stats(
    blockchain: &Blockchain,
) -> ChainStats {
    let contributors: HashSet<_> =
        blockchain
            .chain
            .iter()
            .map(|block| {
                block.contributor
                    .clone()
            })
            .collect();

    ChainStats {
        total_blocks:
            blockchain.chain.len(),

        total_contributors:
            contributors.len(),
    }
}