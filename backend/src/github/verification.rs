use crate::github::{
    commits::GithubCommit,
    issues::GithubIssue,
    pull_requests::PullRequest,
};

pub fn verify_pull_request(
    pull_request: &PullRequest,
) -> bool {
    pull_request.state == "open"
        || pull_request.state == "closed"
}

pub fn verify_issue(
    issue: &GithubIssue,
) -> bool {
    issue.state == "open"
        || issue.state == "closed"
}

pub fn verify_commit(
    commit: &GithubCommit,
) -> bool {
    !commit.sha.is_empty()
}