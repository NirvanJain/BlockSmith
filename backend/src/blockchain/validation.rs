use crate::blockchain::{
    chain::Blockchain,
    hashing::generate_hash,
};

pub fn validate_chain(
    blockchain: &Blockchain,
) -> bool {
    let chain = &blockchain.chain;

    for i in 1..chain.len() {
        let current = &chain[i];
        let previous = &chain[i - 1];

        // Check hash linking
        if current.previous_hash
            != previous.hash
        {
            return false;
        }

        // Recalculate hash
        let recalculated_hash =
            generate_hash( 
                current.index,
                &current.timestamp,
                &current.contributor,
                &current.repository,
                &current.contribution_type,
                &current.contribution_link,
                &current.previous_hash,
            );

        // Check integrity
        if current.hash != recalculated_hash {
            return false;
        }
    }

    true
}