use serde::{Deserialize, Serialize};

use crate::github::github_api::{
    github_base_url,
    github_client,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: u64,
    pub title: String,
    pub state: String,
    pub html_url: String,
}

pub async fn fetch_pull_requests(
    owner: &str,
    repo: &str,
) -> Result<Vec<PullRequest>, reqwest::Error> {
    let client = github_client();

    let url = format!(
        "{}/repos/{}/{}/pulls",
        github_base_url(),
        owner,
        repo
    );

    let response = client
        .get(url)
        .send()
        .await?;

    let pulls = response
        .json::<Vec<PullRequest>>()
        .await?;

    Ok(pulls)
}