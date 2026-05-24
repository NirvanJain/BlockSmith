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
pub struct GithubRelease {
    pub id: u64,
    pub tag_name: String,
    pub name: String,
    pub published_at: String,
}

pub async fn fetch_releases(
    owner: &str,
    repo: &str,
) -> Result<
    Vec<GithubRelease>,
    reqwest::Error,
> {
    let client = github_client();

    let url = format!(
        "{}/repos/{}/{}/releases",
        github_base_url(),
        owner,
        repo
    );

    let response = client
        .get(url)
        .send()
        .await?;

    let releases = response
        .json::<Vec<GithubRelease>>()
        .await?;

    Ok(releases)
}