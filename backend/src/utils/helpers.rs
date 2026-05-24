use chrono::Utc;

pub fn current_timestamp() -> String {
    Utc::now().to_rfc3339()
}

pub fn truncate_hash(
    hash: &str,
) -> String {
    if hash.len() <= 12 {
        hash.to_string()
    } else {
        format!(
            "{}...",
            &hash[..12]
        )
    }
}

pub fn format_repository_name(
    owner: &str,
    repo: &str,
) -> String {
    format!("{}/{}", owner, repo)
}