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

pub type RamCache = LruCache<Game, Vec<Solution>>;

/// Construct and return a least-recently-used cache.
pub fn cache_new(capacity: usize) -> RamCache {
    RamCache::new(capacity)
}

/// Returns a vector of solutions (0 or more).
pub fn cache_get(cache: &mut RamCache, game: &Game) -> Vec<Solution> {
    match cache.get_mut(game) {
        Some(solutions) => solutions.clone(),
        None => vec![],
    }
}

/// Cache a (Game + Solution) key-value pair.
pub fn cache_insert(cache: &mut RamCache, game: &Game, sols: &Vec<Solution>) {
    cache.insert(*game, sols.clone());
}

pub fn cache_print(cache: &RamCache) {
    println!("Cache utilization: {} of {}\n", cache.len(), cache.capacity());
}
