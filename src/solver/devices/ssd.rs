/// SSD Device.

use data::*;
use postgres::Connection;
use postgres::stmt::Statement;
use solver::*;
use solver::db::*;
use solver::ram_cache::*;
use std::cell::RefCell;

pub struct SSD {}

// Cache Sizes
//
// If each item requires 100 bytes (just a guess) then:
//
// cache items    memory size
// -----------    -----------
//      10_000       0.976 MB
//     100_000       9.534 MB
//   1_000_000      95.367 MB
//  10_000_000     953.674 MB
//  20_000_000    1907.349 MB
// 100_000_000    9536.743 MB
pub const CACHE_1_CAP: usize =      1_000;
pub const CACHE_2_CAP: usize = 50_000_000;

pub const CREATE_TABLE: bool = false;
pub const CREATE_INDEXES: bool = false;
pub const TRUNCATE_TABLE: bool = false;

impl SSD {
    pub fn new<'c>(conn: &'c Connection) -> Device<'c> {
        let read_stmt: Statement = db_read_stmt(conn);
        let write_stmt: Statement = db_write_stmt(conn);
        if CREATE_TABLE { db_create_table(conn); }
        if CREATE_INDEXES { db_create_indexes(conn); }
        if TRUNCATE_TABLE { db_truncate_table(conn); }
        Device {
            compute: SSD::compute,
            read: SSD::read,
            write: SSD::write,
            flush: SSD::flush,
            has_compute: false,
            has_read: true,
            has_write: true,
            has_flush: true,
            cache_1: Some(RefCell::new(cache_new(CACHE_1_CAP))),
            cache_2: Some(RefCell::new(cache_new(CACHE_2_CAP))),
            stats: Some(RefCell::new([0; MAX_DEPTH])),
            conn: Some(conn),
            read_stmt: Some(read_stmt),
            write_stmt: Some(write_stmt),
        }
    }

    #[allow(unused_variables)]
    fn compute(game: &Game, depth: Count, stack: &Stack) -> Vec<Solution> {
        panic!("E18XX");
    }

    /// Try reading from cache_1, then cache_2, then from the SSD.
    fn read(device: &Device, game: &Game) -> Vec<Solution> {
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
                                match device.read_stmt {
                                    Some(ref stmt) => db_read(stmt, game),
                                    _ => panic!("E18XX"),
                                }
                            } else {
                                solutions_2
                            }
                        },
                    }
                } else { solutions_1 }
            },
        }
    }

    /// Write to cache_1. If it overflows, write to cache_2 and SSD. Returns
    /// true unless the write to SSD fails.
    fn write(device: &Device, game: &Game, solutions: &Vec<Solution>) -> bool {
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
                                    maybe_write(device, &game_, &solutions_)
                                },
                            }
                        },
                    }
                } else { true };
                cache_insert(mut_cache_1, game, solutions);
                result
            },
        }
    }

    // Drain cache_1 and write to SSD. This is useful if the program gets
    // interrupted. Returns a (success, write_count) tuple.
    pub fn flush(device: &Device) -> (bool, u32) {
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
                            if !maybe_write(device, &game, &solutions) {
                                success = false;
                            }
                        },
                    }
                }
                (success, count)
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

fn maybe_write(device: &Device, game: &Game, sols: &Vec<Solution>) -> bool {
    match device.stats {
        Some(ref stats) => {
            let (turns, unknown) = turns_and_unknown(sols);
            let mut mut_stats = &mut *stats.borrow_mut();
            if save_to_db(turns, unknown, mut_stats) {
                mut_stats[turns as usize] += 1;
                match device.write_stmt {
                    Some(ref stmt) => db_write(stmt, game, sols),
                    _ => panic!("E18XX"),
                }
            } else { true }
        },
        None => panic!("E18XX"),
    }
}

/// Should the SSD write a solution of 'depth' turns given the current
/// statistical information?
fn save_to_db(turns: i16, unknown: bool, stats: &mut [u32; MAX_DEPTH]) -> bool {
    if unknown { return false; }
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

const LOW_MARK: u32 = 200;
const MULTIPLIER: u32 = 20000;

fn threshold(min: u32) -> u32 {
    if min < LOW_MARK {
        LOW_MARK * MULTIPLIER
    } else {
        min * MULTIPLIER
    }
}
