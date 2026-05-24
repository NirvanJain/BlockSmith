use serde::{Deserialize, Serialize};

use crate::github::github_api::{
    github_base_url,
    github_client,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitAuthor {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitInfo {
    pub author: CommitAuthor,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubCommit {
    pub sha: String,
    pub commit: CommitInfo,
}

pub async fn fetch_commits(
    owner: &str,
    repo: &str,
) -> Result<Vec<GithubCommit>, reqwest::Error> {
    let client = github_client();

    let url = format!(
        "{}/repos/{}/{}/commits",
        github_base_url(),
        owner,
        repo
    );

    let response = client
        .get(url)
        .send()
        .await?;

    let commits = response
        .json::<Vec<GithubCommit>>()
        .await?;

    Ok(commits)
}