/// The 'solver' module.

// Include these submodules into this module.
pub use self::device::*;
pub use self::devices::*;
pub use self::outcome::*;
pub use self::solution::*;
pub use self::solve::*;
pub use self::stack::*;
pub use self::stacks::*;

mod device;
mod devices;
mod outcome;
mod solution;
mod solve;
mod stack;
mod stacks;

// Expose and keep these sub-modules (distinct) below this module.
pub mod ram_cache;
pub mod db;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod benchmarks;
