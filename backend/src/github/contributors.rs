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
pub struct Contributor {
    pub login: String,
    pub contributions: u64,
    pub avatar_url: String,
}

pub async fn fetch_contributors(
    owner: &str,
    repo: &str,
) -> Result<
    Vec<Contributor>,
    reqwest::Error,
> {
    let client = github_client();

    let url = format!(
        "{}/repos/{}/{}/contributors",
        github_base_url(),
        owner,
        repo
    );

    let response = client
        .get(url)
        .send()
        .await?;

    let contributors = response
        .json::<Vec<Contributor>>()
        .await?;

    Ok(contributors)
}