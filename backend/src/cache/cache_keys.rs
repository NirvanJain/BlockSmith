pub fn repo_key(
    owner: &str,
    repo: &str,
) -> String {
    format!("repo:{}:{}", owner, repo)
}

pub fn pull_request_key(
    owner: &str,
    repo: &str,
) -> String {
    format!("prs:{}:{}", owner, repo)
}

pub fn issues_key(
    owner: &str,
    repo: &str,
) -> String {
    format!("issues:{}:{}", owner, repo)
}

pub fn commits_key(
    owner: &str,
    repo: &str,
) -> String {
    format!("commits:{}:{}", owner, repo)
}

pub fn user_key(
    username: &str,
) -> String {
    format!("user:{}", username)
}