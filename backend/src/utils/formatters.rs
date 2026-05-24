pub fn format_number(
    number: i64,
) -> String {
    number.to_string()
}

pub fn short_hash(
    hash: &str,
) -> String {
    hash
        .chars()
        .take(10)
        .collect()
}

pub fn format_percentage(
    value: f64,
) -> String {
    format!("{:.2}%", value)
}