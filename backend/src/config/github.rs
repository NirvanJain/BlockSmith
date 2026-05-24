use std::env;

#[derive(Debug, Clone)]
pub struct GithubConfig {
    pub client_id: String,
    pub client_secret: String,
}

impl GithubConfig {
    pub fn from_env() -> Self {
        Self {
            client_id: env::var(
                "GITHUB_CLIENT_ID",
            )
            .unwrap_or_default(),

            client_secret: env::var(
                "GITHUB_CLIENT_SECRET",
            )
            .unwrap_or_default(),
        }
    }
}