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
pub struct Organization {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub description: Option<String>,
}

pub async fn fetch_organization(
    org: &str,
) -> Result<
    Organization,
    reqwest::Error,
> {
    let client = github_client();

    let url = format!(
        "{}/orgs/{}",
        github_base_url(),
        org
    );

    let response = client
        .get(url)
        .send()
        .await?;

    let organization = response
        .json::<Organization>()
        .await?;

    Ok(organization)
}