use data::*;
use postgres::Connection;
use postgres::stmt::Statement;
use solver::*;
use solver::ram_cache::*;
use std::cell::RefCell;

pub const MAX_DEPTH: usize = 81;

/// Device capabilities.
pub struct Device<'c> {
    /// Compute one or more solutions to the specified depth.
    pub compute: fn(&Game, Count, &Stack) -> Vec<Solution>,

    /// Read one or more solutions from the device.
    pub read: fn(&Device, &Game) -> Vec<Solution>,

    /// Write one or more solution to the device.
    pub write: fn(&Device, &Game, &Vec<Solution>) -> bool,

    /// Flush any cached solutions to the device.
    pub flush: fn(&Device) -> (bool, u32),

    /// Supports the 'compute' function?
    pub has_compute: bool,

    /// Supports the 'read' function?
    pub has_read: bool,

    /// Supports the 'write' function?
    pub has_write: bool,

    /// Supports the 'flush' function?
    pub has_flush: bool,

    /// An optional (small) RAM cache.
    pub cache_1: Option<RefCell<RamCache>>,

    /// An optional (large) RAM cache.
    pub cache_2: Option<RefCell<RamCache>>,

    /// An array where the index=solver_depth and value=count.
    pub stats: Option<RefCell<[u32; MAX_DEPTH]>>,

    /// An optional PostgreSQL database connection.
    pub conn: Option<&'c Connection>,

    /// An optional PostgreSQL prepared statement for device reads.
    pub read_stmt: Option<Statement<'c>>,

    /// An optional PostgreSQL prepared statement for device writes.
    pub write_stmt: Option<Statement<'c>>,
}
