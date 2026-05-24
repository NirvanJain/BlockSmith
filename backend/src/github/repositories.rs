use serde::{Deserialize, Serialize};

use crate::github::github_api::{
    github_base_url,
    github_client,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryOwner {
    pub login: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub stargazers_count: u64,
    pub forks_count: u64,
    pub open_issues_count: u64,
    pub html_url: String,
    pub owner: RepositoryOwner,
}

pub async fn fetch_repository(
    owner: &str,
    repo: &str,
) -> Result<Repository, reqwest::Error> {
    let client = github_client();

    let url = format!(
        "{}/repos/{}/{}",
        github_base_url(),
        owner,
        repo
    );

    let response = client
        .get(url)
        .send()
        .await?;

    let repository = response
        .json::<Repository>()
        .await?;

    Ok(repository)
}

pub async fn repository_exists(
    owner: &str,
    repo: &str,
) -> bool {
    fetch_repository(owner, repo)
        .await
        .is_ok()
}