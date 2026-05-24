use crate::cache::ttl_cache::TtlCache;

pub fn cleanup_cache(
    cache: &TtlCache,
) {
    println!(
        "Running cache cleanup..."
    );

    cache.clear();

    println!(
        "Expired cache removed"
    );
}