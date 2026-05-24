use sha2::{Digest, Sha256};

pub fn mine_block(
    data: &str,
    difficulty: usize,
) -> (String, u64) {
    let prefix =
        "0".repeat(difficulty);

    let mut nonce = 0;

    loop {
        let input =
            format!("{}{}", data, nonce);

        let mut hasher = Sha256::new();

        hasher.update(input);

        let hash =
            hex::encode(hasher.finalize());

        if hash.starts_with(&prefix) {
            return (hash, nonce);
        }

        nonce += 1;
    }
}