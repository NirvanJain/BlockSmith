use serde::{Deserialize, Serialize};

use crate::github::github_api::{
    github_base_url,
    github_client,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubIssue {
    pub id: u64,
    pub title: String,
    pub state: String,
    pub html_url: String,
}

pub async fn fetch_issues(
    owner: &str,
    repo: &str,
) -> Result<Vec<GithubIssue>, reqwest::Error> {
    let client = github_client();

    let url = format!(
        "{}/repos/{}/{}/issues",
        github_base_url(),
        owner,
        repo
    );

    let response = client
        .get(url)
        .send()
        .await?;

    let issues = response
        .json::<Vec<GithubIssue>>()
        .await?;

    Ok(issues)
}