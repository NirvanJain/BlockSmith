use std::env;

pub fn github_oauth_url() -> String {
    let client_id = env::var(
        "GITHUB_CLIENT_ID",
    )
    .unwrap_or_default();

    format!(
        "https://github.com/login/oauth/authorize?client_id={}",
        client_id
    )
}

pub fn exchange_code_for_token(
    code: &str,
) -> String {
    format!(
        "github_access_token_{}",
        code
    )
}