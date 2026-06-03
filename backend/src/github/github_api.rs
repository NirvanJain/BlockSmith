use octocrab::Octocrab;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GithubProfile {
    pub login: String,
    pub id: i64,
    pub avatar_url: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub blog: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GithubRepo {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stargazers_count: i32,
    pub forks_count: i32,
    pub owner: GithubOwner,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GithubOwner {
    pub login: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GithubEvent {
    pub id: String,
    pub r#type: String,
    pub repo_name: String,
    pub title: String,
    pub url: String,
    pub created_at: String,
}

pub async fn fetch_github_profile(token: &str) -> Result<GithubProfile, String> {
    if token.starts_with("mock_") || token == "test-token" {
        return Ok(GithubProfile {
            login: "octocat".to_string(),
            id: 5832347,
            avatar_url: "https://avatars.githubusercontent.com/u/5832347?v=4".to_string(),
            email: Some("octocat@github.com".to_string()),
            name: Some("The Octocat".to_string()),
            bio: Some("Testing GitHub APIs locally".to_string()),
            company: Some("GitHub".to_string()),
            location: Some("San Francisco".to_string()),
            blog: Some("https://github.blog".to_string()),
        });
    }

    let octo = Octocrab::builder()
        .personal_token(token.to_string())
        .build()
        .map_err(|e| e.to_string())?;

    let profile: GithubProfile = octo.get("/user", None::<&()>).await.map_err(|e| e.to_string())?;
    Ok(profile)
}

pub async fn fetch_user_repos(token: &str) -> Result<Vec<GithubRepo>, String> {
    if token.starts_with("mock_") || token == "test-token" {
        return Ok(vec![
            GithubRepo {
                id: 1024,
                name: "hello-world".to_string(),
                full_name: "octocat/hello-world".to_string(),
                description: Some("My first repository".to_string()),
                language: Some("Rust".to_string()),
                stargazers_count: 42,
                forks_count: 5,
                owner: GithubOwner { login: "octocat".to_string() },
            }
        ]);
    }

    let octo = Octocrab::builder()
        .personal_token(token.to_string())
        .build()
        .map_err(|e| e.to_string())?;

    let repos = octo.current().list_repos_for_authenticated_user()
        .type_("owner")
        .sort("updated")
        .per_page(50)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for repo in repos {
        result.push(GithubRepo {
            id: repo.id.0 as i64,
            name: repo.name,
            full_name: repo.full_name.unwrap_or_default(),
            description: repo.description,
            language: repo.language.and_then(|v| v.as_str().map(|s| s.to_string())),
            stargazers_count: repo.stargazers_count.unwrap_or(0) as i32,
            forks_count: repo.forks_count.unwrap_or(0) as i32,
            owner: GithubOwner {
                login: repo.owner.map(|o| o.login).unwrap_or_default(),
            },
        });
    }

    Ok(result)
}

pub async fn fetch_user_events(token: &str, username: &str) -> Result<Vec<GithubEvent>, String> {
    if token.starts_with("mock_") || token == "test-token" {
        return Ok(vec![
            GithubEvent {
                id: "evt_1".to_string(),
                r#type: "PullRequestEvent".to_string(),
                repo_name: "octocat/hello-world".to_string(),
                title: "Merge cool features".to_string(),
                url: "https://github.com/octocat/hello-world/pull/1".to_string(),
                created_at: "2026-06-03T00:00:00Z".to_string(),
            },
            GithubEvent {
                id: "evt_2".to_string(),
                r#type: "IssuesEvent".to_string(),
                repo_name: "octocat/hello-world".to_string(),
                title: "Bug: App crash on startup".to_string(),
                url: "https://github.com/octocat/hello-world/issues/2".to_string(),
                created_at: "2026-06-02T12:00:00Z".to_string(),
            }
        ]);
    }

    let client = reqwest::Client::builder()
        .user_agent("OpenHub")
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            let mut auth_val = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap();
            auth_val.set_sensitive(true);
            headers.insert(reqwest::header::AUTHORIZATION, auth_val);
            headers
        })
        .build()
        .map_err(|e| e.to_string())?;

    let url = format!("https://api.github.com/users/{}/events/public", username);
    let res = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !res.status().is_success() {
        return Err(format!("GitHub API event fetch failed with status: {}", res.status()));
    }

    let events_json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let mut events = Vec::new();

    if let Some(arr) = events_json.as_array() {
        for item in arr {
            let id = item.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let r#type = item.get("type").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let repo_name = item.get("repo").and_then(|v| v.get("name")).and_then(|v| v.as_str()).unwrap_or("").to_string();
            
            let title = match r#type.as_str() {
                "PullRequestEvent" => {
                    item.get("payload")
                        .and_then(|p| p.get("pull_request"))
                        .and_then(|pr| pr.get("title"))
                        .and_then(|t| t.as_str())
                        .unwrap_or("Pull Request Event")
                }
                "IssuesEvent" => {
                    item.get("payload")
                        .and_then(|p| p.get("issue"))
                        .and_then(|i| i.get("title"))
                        .and_then(|t| t.as_str())
                        .unwrap_or("Issue Event")
                }
                "PushEvent" => {
                    item.get("payload")
                        .and_then(|p| p.get("commits"))
                        .and_then(|c| c.as_array())
                        .and_then(|c| c.first())
                        .and_then(|c| c.get("message"))
                        .and_then(|m| m.as_str())
                        .unwrap_or("Code Push")
                }
                _ => "GitHub Event",
            }.to_string();

            let url = match r#type.as_str() {
                "PullRequestEvent" => {
                    item.get("payload")
                        .and_then(|p| p.get("pull_request"))
                        .and_then(|pr| pr.get("html_url"))
                        .and_then(|u| u.as_str())
                        .unwrap_or("")
                }
                "IssuesEvent" => {
                    item.get("payload")
                        .and_then(|p| p.get("issue"))
                        .and_then(|i| i.get("html_url"))
                        .and_then(|u| u.as_str())
                        .unwrap_or("")
                }
                _ => "",
            }.to_string();

            let created_at = item.get("created_at").and_then(|v| v.as_str()).unwrap_or("").to_string();

            events.push(GithubEvent {
                id,
                r#type,
                repo_name,
                title,
                url,
                created_at,
            });
        }
    }

    Ok(events)
}

pub fn github_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("BlockSmith")
        .build()
        .unwrap()
}

pub fn github_base_url() -> String {
    "https://api.github.com".to_string()
}