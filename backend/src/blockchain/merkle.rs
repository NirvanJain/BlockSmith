use sha2::{
    Digest,
    Sha256,
};

pub fn hash_data(
    data: &str,
) -> String {
    let mut hasher = Sha256::new();

    hasher.update(data);

    hex::encode(hasher.finalize())
}

pub fn generate_merkle_root(
    transactions: &[String],
) -> String {
    if transactions.is_empty() {
        return hash_data("empty");
    }

    let mut hashes: Vec<String> =
        transactions
            .iter()
            .map(|tx| hash_data(tx))
            .collect();

    while hashes.len() > 1 {
        let mut next_level =
            Vec::new();

        for pair in hashes.chunks(2) {
            if pair.len() == 2 {
                let combined =
                    format!(
                        "{}{}",
                        pair[0],
                        pair[1]
                    );

                next_level.push(
                    hash_data(
                        &combined,
                    ),
                );
            } else {
                next_level.push(
                    pair[0].clone(),
                );
            }
        }

        hashes = next_level;
    }

    hashes[0].clone()
}