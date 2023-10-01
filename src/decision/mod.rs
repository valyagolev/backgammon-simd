use rand::seq::SliceRandom;

use crate::types::{board::Board, prim::Bw};

pub trait MoveDecision {
    fn choose(&mut self, player: Bw, moves: Vec<Board>) -> Board;
}

pub struct RandomMoveDecision<R>(pub R);

impl<R: rand::Rng> MoveDecision for RandomMoveDecision<R> {
    fn choose(&mut self, _player: Bw, moves: Vec<Board>) -> Board {
        moves.choose(&mut self.0).unwrap().clone()
    }
}
