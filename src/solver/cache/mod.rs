/// Cache.

use data::*;
use solver::*;
use lru_cache::LruCache;

pub type Cache = LruCache<Game, Solution>;

/// Construct and return a least-recently-used cache.
pub fn new_cache(capacity: usize) -> Cache {
    Cache::new(capacity)
}

/// Cache a (Game + Solution) key-value pair.
pub fn cache_game_solution(cache: &mut Cache, game: &Game, sol: Solution) {
    cache.insert(*game, sol);
}

pub fn p_cache(cache: &Cache) {
    println!("Cache utilization: {} of {}\n", cache.len(), cache.capacity());
}
