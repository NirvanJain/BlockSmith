use crate::cache::{
    cache_keys::{
        commits_key,
        issues_key,
        pull_request_key,
        repo_key,
    },
    memory_cache::MemoryCache,
};

pub fn invalidate_repository_cache(
    cache: &MemoryCache,
    owner: &str,
    repo: &str,
) {
    let key = repo_key(owner, repo);

    cache.delete(&key);

    println!(
        "Repository cache invalidated"
    );
}

pub fn invalidate_pull_requests_cache(
    cache: &MemoryCache,
    owner: &str,
    repo: &str,
) {
    let key =
        pull_request_key(owner, repo);

    cache.delete(&key);

    println!(
        "PR cache invalidated"
    );
}

pub fn invalidate_issues_cache(
    cache: &MemoryCache,
    owner: &str,
    repo: &str,
) {
    let key = issues_key(owner, repo);

    cache.delete(&key);

    println!(
        "Issues cache invalidated"
    );
}

pub fn invalidate_commits_cache(
    cache: &MemoryCache,
    owner: &str,
    repo: &str,
) {
    let key =
        commits_key(owner, repo);

    cache.delete(&key);

    println!(
        "Commits cache invalidated"
    );
}