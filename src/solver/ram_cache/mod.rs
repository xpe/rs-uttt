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

#[allow(dead_code)]
struct RamCache(LruCache<Game, Solution>);
