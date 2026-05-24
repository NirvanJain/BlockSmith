use crate::cache::{
    cache_keys::{
        commits_key,
        issues_key,
        pull_request_key,
        repo_key,
    },
    memory_cache::MemoryCache,
};

pub fn cache_repository(
    cache: &MemoryCache,
    owner: &str,
    repo: &str,
    data: String,
) {
    let key = repo_key(owner, repo);

    cache.set(key, data);
}

pub fn cache_pull_requests(
    cache: &MemoryCache,
    owner: &str,
    repo: &str,
    data: String,
) {
    let key =
        pull_request_key(owner, repo);

    cache.set(key, data);
}

pub fn cache_issues(
    cache: &MemoryCache,
    owner: &str,
    repo: &str,
    data: String,
) {
    let key = issues_key(owner, repo);

    cache.set(key, data);
}

pub fn cache_commits(
    cache: &MemoryCache,
    owner: &str,
    repo: &str,
    data: String,
) {
    let key = commits_key(owner, repo);

    cache.set(key, data);
}