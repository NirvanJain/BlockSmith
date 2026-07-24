use std::time::Instant;

use blocksmith::blockchain::chain::Blockchain;

#[test]
fn test_block_creation_performance() {
    let start =
        Instant::now();

    let mut blockchain =
        Blockchain::new();

    for i in 0..100 {
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

    let duration =
        start.elapsed();

    println!(
        "100 blocks created in {:?}",
        duration
    );

    assert!(
        duration.as_secs() < 5
    );
}