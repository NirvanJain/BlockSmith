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
pub struct RateLimit {
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct RateLimitCore {
    pub core: RateLimit,
}

pub async fn fetch_rate_limits(
) -> Result<
    RateLimitCore,
    reqwest::Error,
> {
    let client = github_client();

    let url = format!(
        "{}/rate_limit",
        github_base_url()
    );

    let response = client
        .get(url)
        .send()
        .await?;

    let limits = response
        .json::<RateLimitCore>()
        .await?;

    Ok(limits)
}