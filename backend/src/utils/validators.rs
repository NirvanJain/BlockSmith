pub fn validate_github_username(
    username: &str,
) -> bool {
    !username.trim().is_empty()
        && username.len() >= 3
}

pub fn validate_repository(
    repository: &str,
) -> bool {
    repository.contains("/")
}

pub fn validate_contribution_type(
    contribution_type: &str,
) -> bool {
    matches!(
        contribution_type,
        "pull_request"
            | "issue"
            | "commit"
    )
}