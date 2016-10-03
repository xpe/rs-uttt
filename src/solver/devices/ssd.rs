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
pub const CACHE_CAP: usize = 20_000_000;

impl SSD {
    pub fn new() -> Device {
        let pool = pool_new(CONN_STR);
        let conn = pool.get().expect("Error 1865");
        db_create_table(&conn);
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
    fn compute(game: &Game, depth: Count, stack: &Stack) -> Option<Solution> {
        unimplemented!();
    }

    fn read(device: &Device, game: &Game) -> Option<Solution> {
        match device.pool {
            Some(ref pool) => {
                match device.cache {
                    Some(ref cache) => {
                        let mut mut_cache = &mut *cache.borrow_mut();
                        match cache_get(mut_cache, game) {
                            Some(sol) => Some(sol),
                            None => pool_read(pool, game),
                        }
                    }
                    None => panic!("Error 1866"),
                }
            },
            None => panic!("Error 6523"),
        }
    }

    fn write(device: &Device, game: &Game, solution: Solution) -> bool {
        match device.pool {
            Some(ref pool) => {
                match device.cache {
                    Some(ref cache) => {
                        let mut mut_cache = &mut *cache.borrow_mut();
                        cache_insert(mut_cache, game, solution);
                        pool_write(pool, game, solution)
                    },
                    None => panic!("Error 3375"),
                }
            },
            None => panic!("Error 0401"),
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
