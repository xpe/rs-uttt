/// SSD Device.

use data::*;
use solver::*;
use solver::db::*;
use solver::ram_cache::*;
use std::cell::RefCell;

pub const CONN_STR: &'static str = "postgres://xpe:xpe_0987@localhost";

pub struct SSD {}

/// If each item requires 100 bytes (just a guess) then:
///
/// cache items    memory size
/// -----------    -----------
///      10_000       0.976 MB
///     100_000       9.534 MB
///   1_000_000      95.367 MB
///  10_000_000     953.674 MB
///  20_000_000    1907.349 MB
/// 100_000_000    9536.743 MB
pub const CACHE_CAP: usize = 25_000_000;

impl SSD {
    pub fn new() -> Device {
        let pool = pool_new(CONN_STR);
        let conn = pool.get().expect("E1801");
        db_create_table(&conn);
        db_create_indexes(&conn);
        // db_truncate_table(&conn);
        Device {
            compute: SSD::compute,
            read: SSD::read,
            write: SSD::write,
            has_compute: false,
            has_read: true,
            has_write: true,
            pool: Some(pool),
            cache: Some(RefCell::new(cache_new(CACHE_CAP))),
        }
    }

    #[allow(unused_variables)]
    fn compute(game: &Game, depth: Count, stack: &Stack) -> Vec<Solution> {
        unimplemented!();
    }

    fn read(device: &Device, game: &Game) -> Vec<Solution> {
        match device.pool {
            Some(ref pool) => {
                match device.cache {
                    Some(ref cache) => {
                        let mut mut_cache = &mut *cache.borrow_mut();
                        let solutions = cache_get(mut_cache, game);
                        if solutions.is_empty() {
                            pool_read(pool, game)
                        } else {
                            solutions
                        }
                    }
                    None => panic!("E1802"),
                }
            },
            None => panic!("E1803"),
        }
    }

    fn write(device: &Device, game: &Game, solutions: &Vec<Solution>) -> bool {
        match device.pool {
            Some(ref pool) => {
                match device.cache {
                    Some(ref cache) => {
                        let mut mut_cache = &mut *cache.borrow_mut();
                        cache_insert(mut_cache, game, solutions);
                        pool_write(pool, game, solutions)
                    },
                    None => panic!("E1804"),
                }
            },
            None => panic!("E1805"),
        }
    }

    pub fn cache_len(device: &Device) -> usize {
        match device.cache {
            Some(ref cache) => {
                let cache = & *cache.borrow();
                cache.len()
            },
            None => 0,
        }
    }
}
