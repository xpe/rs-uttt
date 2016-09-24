use data::Game;
use solver::solution::Solution;

/// A device always can read (get) and sometimes can write (put). If `put` is
/// supported, `has_put` must report `true`.
pub trait Device {
    /// Reads from the device.
    fn get(&self, game: Game) -> Option<Solution>;

    /// Writes to the device.
    fn put(&self, game: Game, solution: Solution) -> bool;

    /// Returns true if put is supported, false otherwise.
    fn has_put(&self) -> bool;

    /// Returns a human-presentable string.
    fn label(&self) -> &str;
}
