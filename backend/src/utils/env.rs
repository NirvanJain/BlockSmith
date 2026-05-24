use std::env;

pub fn get_env(
    key: &str,
) -> String {
    env::var(key)
        .unwrap_or_else(|_| {
            panic!(
                "Environment variable {} not found",
                key
            )
        })
}

pub fn database_url() -> String {
    get_env("DATABASE_URL")
}

pub fn jwt_secret() -> String {
    get_env("JWT_SECRET")
}

pub fn github_client_id() -> String {
    get_env("GITHUB_CLIENT_ID")
}

pub fn github_client_secret() -> String {
    get_env("GITHUB_CLIENT_SECRET")
}