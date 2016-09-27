use data::{Count, Game};
use solver::{Solution, Stack};

/// A solver device; e.g. RAM, SSD, HDD, or CPU.
pub trait Device {
    /// Compute a solution to the specified depth.
    fn compute(&self, game: &Game, depth: Count, stack: &Stack)
               -> Option<Solution>;

    /// Read a solution from the device.
    fn read(&self, game: &Game) -> Option<Solution>;

    /// Write a solution from the device.
    fn write(&self, game: &Game, solution: Solution) -> bool;

    /// Does the device support the `compute` function?
    fn supports_compute(&self) -> bool;

    /// Does the device support the `read` function?
    fn supports_read(&self) -> bool;

    /// Does the device support the `write` function?
    fn supports_write(&self) -> bool;

    /// Returns a human-presentable label for the device.
    fn label(&self) -> &str;
}
