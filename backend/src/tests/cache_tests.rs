use blocksmith::cache::memory_cache::MemoryCache;

#[test]
fn test_cache_set_and_get() {
    let cache =
        MemoryCache::new();

    cache.set(
        "user".to_string(),
        "nirvanjain".to_string(),
    );

    let value =
        cache.get("user");

    assert_eq!(
        value.unwrap(),
        "nirvanjain"
    );
}