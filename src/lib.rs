#![feature(slice_patterns)]
#![cfg_attr(test, feature(test))]

pub mod accessors;
pub mod constants;
pub mod constructors;
pub mod data;
pub mod logic;
pub mod random;
pub mod runners;
pub mod show;
pub mod solver;
pub mod utility;

#[macro_use]
extern crate chan;

extern crate chan_signal;
extern crate lru_cache;
extern crate postgres;
extern crate rand;

#[cfg(test)]
mod tests;

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
extern crate test;
