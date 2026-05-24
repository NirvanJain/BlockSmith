use reqwest::Client;

pub fn github_client() -> Client {
    Client::builder()
        .user_agent("BlockSmith")
        .build()
        .unwrap()
}

pub fn github_base_url() -> String {
    "https://api.github.com".to_string()
}