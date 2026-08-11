use sha2::{Digest, Sha256};

pub fn generate_hash(
    index: u32,
    timestamp: &str,
    contributor: &str,
    repository: &str,
    contribution_type: &str,
    contribution_link: &str,
    previous_hash: &str, 
) -> String {

    let input = format!(
        
        "{}{}{}{}{}{}{}",
        index,
        timestamp,
        contributor,
        repository,
        contribution_type,
        contribution_link,
        previous_hash
    );

    let mut hasher = Sha256::new();

    hasher.update(input);

    let result = hasher.finalize();

    hex::encode(result)
}