use itertools::Itertools;

use crate::{
    types::{
        board::Board,
        dice::{Dice, Die},
        prim::Bw,
    },
    util::timings,
};

pub mod basic;
pub mod simd;

pub trait MoveGen {
    fn gen_moves_one_die(board: &Board, die: Die, player: Bw) -> Vec<Board>;

    fn gen_unique_moves_one_die(board: &Board, die: Die, player: Bw) -> Vec<Board> {
        timings::time(timings::PerfParts::UniqueOneDie, || {
            Self::gen_moves_one_die(board, die, player)
                .into_iter()
                .unique()
                .collect()
        })
    }

    fn gen_unique_moves(board: &Board, dice: Dice, player: Bw) -> Vec<Board> {
        timings::time(timings::PerfParts::UniqueMoves, || {
            if dice.is_double() {
                return timings::time(timings::PerfParts::UniqueMovesDouble, || {
                    let die = dice.0;

                    let mut boards = vec![board.clone()];

                    for _ in 0..4 {
                        let new_boards = boards
                            .iter()
                            .flat_map(|b| Self::gen_moves_one_die(&b, die, player))
                            .unique()
                            .collect::<Vec<_>>();

                        if new_boards.is_empty() {
                            break;
                        } else {
                            boards = new_boards;
                        }
                    }

                    boards
                });
            }

            timings::time(timings::PerfParts::UniqueMovesNormalDice, || {
                let d0 = Self::gen_moves_one_die(board, dice.0, player);
                let d01 = d0
                    .iter()
                    .flat_map(|b| Self::gen_moves_one_die(&b, dice.1, player));
                let d1 = Self::gen_moves_one_die(board, dice.0, player);
                let d10 = d1
                    .iter()
                    .flat_map(|b| Self::gen_moves_one_die(&b, dice.1, player));

                d01.chain(d10).unique().collect()
            })
        })
    }

    // fn gen_all_possible_moves(&self, board: &Board, player: Bw) -> Vec<Vec<Board>> {
    //     let mut moves = Vec::new();

    //     for dice in Dice::iter_all_possible() {
    //         moves.push(Self::gen_moves(board, dice, player));
    //     }

    //     moves
    // }
}
