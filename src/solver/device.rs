use data::{Count, Game};
use solver::Solution;

/// A solver device; e.g. RAM, SSD, HDD, or CPU.
pub trait Device {
    fn read(&self, game: &Game, depth: Count) -> Option<Solution>;

    fn write(&self, game: &Game, solution: Solution) -> bool;

    fn is_writable(&self) -> bool;

    fn label(&self) -> &str;
}
