use uuid::Uuid;

pub fn generate_magic_link_token(
) -> String {
    Uuid::new_v4().to_string()
}

pub fn validate_magic_link(
    token: &str,
) -> bool {
    !token.trim().is_empty()
}