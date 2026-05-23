mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    println!("==============================");
    println!("      BlockSmith Started");
    println!("==============================");

    // Create blockchain
    let mut blockchain = Blockchain::new();

    // Add blocks
    blockchain.add_block(
        "Nirvan sent 5 BTC to Alex".to_string(),
    );

    blockchain.add_block(
        "Alex sent 2 BTC to Sarah".to_string(),
    );

    blockchain.add_block(
        "Sarah minted an NFT".to_string(),
    );

    // Print blockchain
    blockchain.print_chain();

    println!("\n==============================");
    println!(
        "Blockchain Valid: {}",
        blockchain.is_valid()
    );
    println!("==============================");
}