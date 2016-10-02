/// RAM Cache.
///
/// To create a cache with 1,000 key-value pairs:
/// let cache = RamCache::new(1_000);
///
/// To insert an item (where game: &Game, solution: Solution):
/// cache.insert(*game, sol);

use data::*;
use solver::*;
use lru_cache::LruCache;

pub type RamCache = LruCache<Game, Solution>;

/// Construct and return a least-recently-used cache.
pub fn cache_new(capacity: usize) -> RamCache {
    RamCache::new(capacity)
}

/// If present, returns Some(solution). Otherwise returns None.
pub fn cache_get(cache: &mut RamCache, game: &Game) -> Option<Solution> {
    match cache.get_mut(game) {
        Some(sol) => Some(*sol),
        None => None,
    }
}

/// Cache a (Game + Solution) key-value pair.
pub fn cache_insert(cache: &mut RamCache, game: &Game, sol: Solution) {
    cache.insert(*game, sol);
}

pub fn cache_print(cache: &RamCache) {
    println!("Cache utilization: {} of {}\n", cache.len(), cache.capacity());
}
