use chrono::Utc;

use crate::blockchain::block::Block;

pub fn create_genesis_block(
) -> Block {
    Block {
        index: 0,

        timestamp:
            Utc::now()
                .to_rfc3339(),

        contributor:
            "BlockSmith"
                .to_string(),

        repository:
            "Genesis"
                .to_string(),

        contribution_type:
            "Genesis Block"
                .to_string(),

        contribution_link:
            "None"
                .to_string(),

        previous_hash:
            "0".to_string(),

        hash:
            "genesis_hash"
                .to_string(),
    }
}