#![feature(portable_simd)]

pub mod decision;
pub mod game;
pub mod movegen;
pub mod randgen;
pub mod types;
pub mod util;

#[cfg(feature = "backgammon-compat")]
pub mod compat;

#[cfg(test)]
mod tests {}
