pub fn to_slug(
    input: &str,
) -> String {
    input
        .to_lowercase()
        .replace(' ', "-")
}

pub fn truncate(
    input: &str,
    length: usize,
) -> String {
    if input.len() <= length {
        return input.to_string();
    }

    format!(
        "{}...",
        &input[..length]
    )
}

pub fn normalize(
    input: &str,
) -> String {
    input.trim().to_lowercase()
}