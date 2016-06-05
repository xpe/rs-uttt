#![feature(slice_patterns)]
#![cfg_attr(test, feature(test))]

pub mod accessors;
pub mod constants;
pub mod constructors;
pub mod data;
pub mod examples;
pub mod logic;
pub mod random;
pub mod show;
pub mod solver;
pub mod utility;

extern crate rand;
extern crate lru_time_cache;

#[cfg(test)]
mod tests;

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
extern crate test;
