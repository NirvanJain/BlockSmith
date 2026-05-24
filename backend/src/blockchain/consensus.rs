use crate::blockchain::block::Block;

pub trait Consensus {
    fn validate_block(
        &self,
        block: &Block,
    ) -> bool;
}

pub struct ProofOfAuthority;

impl Consensus for ProofOfAuthority {
    fn validate_block(
        &self,
        block: &Block,
    ) -> bool {
        !block.hash.is_empty()
    }
}

pub struct ProofOfWork;

impl Consensus for ProofOfWork {
    fn validate_block(
        &self,
        block: &Block,
    ) -> bool {
        block.hash.starts_with("0000")
    }
}