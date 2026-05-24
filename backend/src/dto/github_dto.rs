use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubPullRequestDto {
    pub id: u64,
    pub title: String,
    pub state: String,
    pub html_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubIssueDto {
    pub id: u64,
    pub title: String,
    pub state: String,
    pub html_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubCommitDto {
    pub sha: String,
    pub message: String,
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubRepositoryDto {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub stars: u64,
    pub forks: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubWebhookDto {
    pub action: String,
    pub repository: String,
    pub contributor: String,
}