use serde::{
    Deserialize,
    Serialize,
};

use crate::github::github_api::{
    github_base_url,
    github_client,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct SearchRepository {
    pub full_name: String,
    pub html_url: String,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct SearchResponse {
    pub total_count: u64,
    pub items:
        Vec<SearchRepository>,
}

pub async fn search_repositories(
    query: &str,
) -> Result<
    SearchResponse,
    reqwest::Error,
> {
    let client = github_client();

    let url = format!(
        "{}/search/repositories?q={}",
        github_base_url(),
        query
    );

    let response = client
        .get(url)
        .send()
        .await?;

    let results = response
        .json::<SearchResponse>()
        .await?;

    Ok(results)
}