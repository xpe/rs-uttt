use data::*;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;
use solver::*;
use solver::ram_cache::*;
use std::cell::RefCell;

/// Device capabilities.
pub struct Device {
    /// Compute one or more solutions to the specified depth.
    pub compute: fn(&Game, Count, &Stack) -> Vec<Solution>,

    /// Read one or more solutions from the device.
    pub read: fn(&Device, &Game) -> Vec<Solution>,

    /// Write one or more solution to the device.
    pub write: fn(&Device, &Game, &Vec<Solution>) -> bool,

    /// Flush any cached solutions to the device.
    pub flush: fn(&Device) -> (bool, usize),

    /// Supports the 'compute' function?
    pub has_compute: bool,

    /// Supports the 'read' function?
    pub has_read: bool,

    /// Supports the 'write' function?
    pub has_write: bool,

    /// Supports the 'flush' function?
    pub has_flush: bool,

    /// An optional R2D2 PostgreSQL database connection pool.
    pub pool: Option<Pool<PostgresConnectionManager>>,

    /// An optional (small) RAM cache.
    pub cache_1: Option<RefCell<RamCache>>,

    /// An optional (large) RAM cache.
    pub cache_2: Option<RefCell<RamCache>>,
}
