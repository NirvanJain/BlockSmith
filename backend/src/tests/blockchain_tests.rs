use blocksmith::blockchain::{
    chain::Blockchain,
    validation::validate_chain,
};

#[test]
fn test_blockchain_creation() {
    let blockchain = Blockchain::new();

    assert_eq!(
        blockchain.chain.len(),
        1
    );
}

#[test]
fn test_add_block() {
    let mut blockchain =
        Blockchain::new();

    blockchain.add_block(
        "nirvanjain".to_string(),
        "BlockSmith".to_string(),
        "pull_request".to_string(),
        "https://github.com/pr/1"
            .to_string(),
    );

    assert_eq!(
        blockchain.chain.len(),
        2
    );
}

#[test]
fn test_blockchain_validation() {
    let mut blockchain =
        Blockchain::new();

    blockchain.add_block(
        "nirvanjain".to_string(),
        "BlockSmith".to_string(),
        "pull_request".to_string(),
        "https://github.com/pr/1"
            .to_string(),
    );

    assert!(validate_chain(
        &blockchain
    ));
}