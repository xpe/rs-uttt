use data::*;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
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

    /// Supports the 'compute' function?
    pub has_compute: bool,

    /// Supports the 'read' function?
    pub has_read: bool,

    /// Supports the 'write' function?
    pub has_write: bool,

    /// An optional R2D2 PostgreSQL database connection pool.
    pub pool: Option<Pool<PostgresConnectionManager>>,

    /// An optional RAM cache.
    pub cache: Option<RefCell<RamCache>>,
}
