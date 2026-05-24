use blocksmith::blockchain::blockchain::Blockchain;

#[test]
fn test_massive_blockchain_stress() {
    let mut blockchain =
        Blockchain::new();

    for i in 0..5000 {
        blockchain.add_block(
            format!("user{}", i),
            "BlockSmith".to_string(),
            "pull_request".to_string(),
            format!(
                "https://github.com/pr/{}",
                i
            ),
        );
    }

    assert_eq!(
        blockchain.chain.len(),
        5001
    );
}