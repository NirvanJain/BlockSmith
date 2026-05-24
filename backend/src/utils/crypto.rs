use sha2::{
    Digest,
    Sha256,
};

use uuid::Uuid;

pub fn sha256_hash(
    input: &str,
) -> String {
    let mut hasher = Sha256::new();

    hasher.update(input);

    format!(
        "{:x}",
        hasher.finalize()
    )
}

pub fn generate_token() -> String {
    Uuid::new_v4().to_string()
}

pub fn generate_api_key() -> String {
    format!(
        "bs_{}",
        Uuid::new_v4()
    )
}