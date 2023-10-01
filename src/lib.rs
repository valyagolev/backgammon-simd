#![feature(portable_simd)]

pub mod decision;
pub mod game;
pub mod movegen;
pub mod randgen;
pub mod types;
pub mod util;

#[cfg(feature = "backgammon-compat")]
pub mod compat;

pub use {
    game::Game, game::GameState, movegen::basic::BasicMoveGenerator,
    movegen::simd::Simd1MoveGenerator, movegen::MoveGen, types::board::Board,
    types::board::BoardCoord, types::dice::Dice, types::dice::Die, types::prim::BAndW,
    types::prim::BOrW, types::prim::Bw,
};

#[cfg(test)]
mod tests {}
