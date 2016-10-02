use data::*;
use postgres::Connection;
use solver::*;

/// Device capabilities.
pub struct Device {
    /// Compute a solution to the specified depth.
    pub compute: fn(&Game, Count, &Stack) -> Option<Solution>,

    /// Read a solution from the device.
    pub read: fn(&Device, &Game) -> Option<Solution>,

    /// Write a solution from the device.
    pub write: fn(&Device, &Game, Solution) -> bool,

    /// Supports the 'compute' function?
    pub has_compute: bool,

    /// Supports the 'read' function?
    pub has_read: bool,

    /// Supports the 'write' function?
    pub has_write: bool,

    /// An optional PostgreSQL database connection.
    pub conn: Option<Connection>,
}
