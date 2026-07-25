use std::env;
use serde::Deserialize;

/// Build the GitHub OAuth authorization URL.
/// `state` is a random token to prevent CSRF.
pub fn github_oauth_url(state: &str) -> String {
    let client_id = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set");
    let redirect_uri = redirect_uri();

    format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=read:user,user:email&state={}",
        client_id,
        urlencoding::encode(&redirect_uri),
        urlencoding::encode(state),
    )
}

/// Exchange the authorization code for an access token via GitHub's OAuth API.
pub async fn exchange_code_for_token(code: &str) -> Result<String, String> {
    let client_id = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set");
    let client_secret = env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET must be set");

    let client = reqwest::Client::new();
    let resp = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", code),
        ])
        .send()
        .await
        .map_err(|e| format!("Failed to send token exchange request: {}", e))?;

    let body: GitHubTokenResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse token response: {}", e))?;

    match body.access_token {
        Some(token) => Ok(token),
        None => Err(format!(
            "GitHub token exchange failed: {}",
            body.error_description.unwrap_or_else(|| body.error.unwrap_or_default())
        )),
    }
}

/// Fetch the authenticated user's GitHub profile using the access token.
pub async fn fetch_github_profile(access_token: &str) -> Result<GitHubUser, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "BlockSmith")
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch GitHub profile: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("GitHub API returned {}: {}", status, text));
    }

    resp.json::<GitHubUser>()
        .await
        .map_err(|e| format!("Failed to parse GitHub profile: {}", e))
}

/// Fetch the user's primary email from GitHub.
pub async fn fetch_github_email(access_token: &str) -> Result<Option<String>, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.github.com/user/emails")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "BlockSmith")
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch GitHub emails: {}", e))?;

    if !resp.status().is_success() {
        return Ok(None);
    }

    let emails: Vec<GitHubEmail> = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse GitHub emails: {}", e))?;

    // Prefer the primary, verified email
    let primary = emails
        .iter()
        .find(|e| e.primary && e.verified)
        .or_else(|| emails.iter().find(|e| e.verified))
        .map(|e| e.email.clone());

    Ok(primary)
}

fn redirect_uri() -> String {
    env::var("GITHUB_REDIRECT_URI")
        .unwrap_or_else(|_| "http://localhost:3000/api/v1/auth/github/callback".to_string())
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GitHubTokenResponse {
    access_token: Option<String>,
    token_type: Option<String>,
    scope: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GitHubUser {
    pub id: i64,
    pub login: String,
    pub avatar_url: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubEmail {
    email: String,
    primary: bool,
    verified: bool,
}
