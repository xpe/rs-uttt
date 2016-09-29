/// The 'solver' module.

// Include these submodules into this module.
pub use self::device::*;
pub use self::devices::*;
pub use self::layer::*;
pub use self::layers::*;
pub use self::outcome::*;
pub use self::policies::*;
pub use self::policy::*;
pub use self::solution::*;
pub use self::solve::*;
pub use self::stack::*;
pub use self::stacks::*;

mod device;
mod devices;
mod layer;
mod layers;
mod outcome;
mod policies;
mod policy;
mod solution;
mod solve;
mod stack;
mod stacks;

// Expose and keep these sub-modules (distinct) below this module.
pub mod cache;
pub mod db;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod benchmarks;
