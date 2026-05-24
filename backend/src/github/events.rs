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
pub struct GithubEvent {
    pub id: String,
    pub r#type: String,
    pub created_at: String,
}

pub async fn fetch_repo_events(
    owner: &str,
    repo: &str,
) -> Result<
    Vec<GithubEvent>,
    reqwest::Error,
> {
    let client = github_client();

    let url = format!(
        "{}/repos/{}/{}/events",
        github_base_url(),
        owner,
        repo
    );

    let response = client
        .get(url)
        .send()
        .await?;

    let events = response
        .json::<Vec<GithubEvent>>()
        .await?;

    Ok(events)
}