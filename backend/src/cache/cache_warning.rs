use crate::cache::{
    cache_keys::repo_key,
    memory_cache::MemoryCache,
};

pub fn warm_repository_cache(
    cache: &MemoryCache,
    owner: &str,
    repo: &str,
) {
    let key = repo_key(owner, repo);

    let value = format!(
        "Preloaded repository cache for {}/{}",
        owner,
        repo
    );

    cache.set(key, value);

    println!(
        "Repository cache warmed"
    );
}

pub fn warm_leaderboard_cache(
    cache: &MemoryCache,
) {
    cache.set(
        "leaderboard".to_string(),
        "cached_leaderboard_data"
            .to_string(),
    );

    println!(
        "Leaderboard cache warmed"
    );
}