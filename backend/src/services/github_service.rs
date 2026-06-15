use crate::github::{
    commits::fetch_commits,
    issues::fetch_issues,
    pull_requests::fetch_pull_requests,
};
 
pub async fn fetch_repo_data(
    owner: &str,
    repo: &str,
) {
    let pulls =
        fetch_pull_requests(owner, repo)
            .await;

    let issues =
        fetch_issues(owner, repo).await;

    let commits =
        fetch_commits(owner, repo).await;

    println!("PRs: {:?}", pulls.is_ok());

    println!(
        "Issues: {:?}", 
        issues.is_ok()
    );

    println!(
        "Commits: {:?}",
        commits.is_ok()
    );
}
