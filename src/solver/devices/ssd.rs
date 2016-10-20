/// SSD Device.
///
/// Cache Sizes
///
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

use data::*;
use solver::*;
use solver::db::*;
use solver::ram_cache::*;
use std::cell::RefCell;

pub const CONN_STR: &'static str = "postgres://xpe:xpe_0987@localhost";

pub struct SSD {}

pub const CACHE_1_CAP: usize =    100_000;
pub const CACHE_2_CAP: usize = 50_000_000;

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
            flush: SSD::flush,
            has_compute: false,
            has_read: true,
            has_write: true,
            has_flush: true,
            pool: Some(pool),
            cache_1: Some(RefCell::new(cache_new(CACHE_1_CAP))),
            cache_2: Some(RefCell::new(cache_new(CACHE_2_CAP))),
            stats: Some(RefCell::new([0; MAX_DEPTH])),
        }
    }

    #[allow(unused_variables)]
    fn compute(game: &Game, depth: Count, stack: &Stack) -> Vec<Solution> {
        unimplemented!();
    }

    /// Try reading from cache_1, then cache_2, then from the SSD.
    fn read(device: &Device, game: &Game) -> Vec<Solution> {
        match device.pool {
            None => panic!("E18XX"),
            Some(ref pool) => {
                match device.cache_1 {
                    None => panic!("E18XX"),
                    Some(ref cache_1) => {
                        let mut mut_cache_1 = &mut *cache_1.borrow_mut();
                        let solutions_1 = cache_get(mut_cache_1, game);
                        if solutions_1.is_empty() {
                            match device.cache_2 {
                                None => panic!("E18XX"),
                                Some(ref cache_2) => {
                                    let mut_cache_2 = &mut *cache_2.borrow_mut();
                                    let solutions_2 = cache_get(mut_cache_2, game);
                                    if solutions_2.is_empty() {
                                        pool_read(pool, game)
                                    } else {
                                        solutions_2
                                    }
                                },
                            }
                        } else { solutions_1 }
                    },
                }
            },
        }
    }

    /// Write to cache_1. If it overflows, write to cache_2 and SSD. Returns
    /// true unless the write to SSD fails.
    fn write(device: &Device, game: &Game, solutions: &Vec<Solution>) -> bool {
        match device.pool {
            None => panic!("E18XX"),
            Some(ref pool) => {
                match device.cache_1 {
                    None => panic!("E18XX"),
                    Some(ref cache_1) => {
                        let mut mut_cache_1 = &mut *cache_1.borrow_mut();
                        // If cache_1 would overflow, remove and write to
                        // cache_2 and SSD.
                        let result = if mut_cache_1.len() == CACHE_1_CAP {
                            match cache_remove_lru(mut_cache_1) {
                                None => panic!("E18XX"),
                                Some((game_, solutions_)) => {
                                    match device.cache_2 {
                                        None => panic!("E18XX"),
                                        Some(ref cache_2) => {
                                            let mut mut_cache_2 = &mut *cache_2.borrow_mut();
                                            cache_insert(mut_cache_2, &game_, &solutions_);
                                            maybe_write(device, pool, &game_, &solutions_)
                                        },

                                    }
                                },
                            }
                        } else { true };
                        cache_insert(mut_cache_1, game, solutions);
                        result
                    },
                }
            },
        }
    }

    // Drain cache_1 and write to SSD. This is useful if the program gets
    // interrupted. Returns a (success, write_count) tuple.
    pub fn flush(device: &Device) -> (bool, u32) {
        match device.pool {
            None => panic!("E18XX"),
            Some(ref pool) => {
                match device.cache_1 {
                    None => panic!("E18XX"),
                    Some(ref cache) => {
                        let mut mut_cache = &mut *cache.borrow_mut();
                        let mut success = true;
                        let mut count: u32 = 0;
                        loop {
                            match cache_remove_lru(mut_cache) {
                                None => break,
                                Some((game, solutions)) => {
                                    count += 1;
                                    if !maybe_write(device, pool, &game, &solutions) {
                                        success = false;
                                    }
                                },
                            }
                        }
                        (success, count)
                    },
                }
            },
        }
    }

    pub fn cache_1_len(device: &Device) -> usize {
        match device.cache_1 {
            None => 0,
            Some(ref cache) => {
                let cache = & *cache.borrow();
                cache.len()
            },
        }
    }

    pub fn cache_2_len(device: &Device) -> usize {
        match device.cache_2 {
            None => 0,
            Some(ref cache) => {
                let cache = & *cache.borrow();
                cache.len()
            },
        }
    }
}

fn maybe_write(device: &Device, pool: &PGPool, game: &Game,
    solutions: &Vec<Solution>) -> bool {
    match device.stats {
        Some(ref stats) => {
            let (turns, unknown) = turns_and_unknown(solutions);
            let mut mut_stats = &mut *stats.borrow_mut();
            if save_to_db(turns, unknown, mut_stats) {
                mut_stats[turns as usize] += 1;
                pool_write(pool, game, solutions)
            } else {
                true
            }
        },
        None => panic!("E18XX"),
    }
}

/// Should the SSD write a solution of 'depth' turns given the current
/// statistical information?
fn save_to_db(turns: i16, unknown: bool,
    stats: &mut [u32; MAX_DEPTH]) -> bool {
    if unknown || turns == 0 {
        return false;
    }
    let mut max: u32 = 0;
    let mut nonzero_min: Option<u32> = None;
    for val in stats.into_iter() {
        if *val != 0 {
            if *val > max { max = *val; }
            match nonzero_min {
                Some(min) => if *val < min { nonzero_min = Some(*val); },
                None => nonzero_min = Some(*val),
            }
        }
    }
    match nonzero_min {
        Some(min) => stats[turns as usize] < threshold(min),
        None => true,
    }
}

fn threshold(min: u32) -> u32 {
    if min < 250 {
        5000
    } else {
        min * 20
    } // TODO: what about overflow?
}
