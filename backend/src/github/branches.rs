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
pub struct Branch {
    pub name: String,
}

pub async fn fetch_branches(
    owner: &str,
    repo: &str,
) -> Result<
    Vec<Branch>,
    reqwest::Error,
> {
    let client = github_client();

    let url = format!(
        "{}/repos/{}/{}/branches",
        github_base_url(),
        owner,
        repo
    );

    let response = client
        .get(url)
        .send()
        .await?;

    let branches = response
        .json::<Vec<Branch>>()
        .await?;

    Ok(branches)
}