#![feature(slice_patterns)]

pub mod accessors;
pub mod constants;
pub mod constructors;
pub mod data;
pub mod examples;
pub mod logic;
pub mod random;
pub mod show;
pub mod utility;

extern crate rand;

#[cfg(test)]
mod tests;

#[cfg(test)]
extern crate quickcheck;
