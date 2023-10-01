use std::simd::{Simd, SimdElement, SimdInt, SimdPartialOrd};

use crate::{
    types::{
        board::{Board, BoardCoord},
        prim::{BOrW, Bw},
    },
    util::timings::{self, PerfParts},
};

use super::MoveGen;

pub struct Simd1MoveGenerator;

#[allow(dead_code)]
#[inline]
fn rotate_dyn<const LEFT: bool, T: SimdElement>(simd: Simd<T, 32>, cnt: usize) -> Simd<T, 32> {
    match (LEFT, cnt) {
        (true, 1) => simd.rotate_lanes_left::<1>(),
        (true, 2) => simd.rotate_lanes_left::<2>(),
        (true, 3) => simd.rotate_lanes_left::<3>(),
        (true, 4) => simd.rotate_lanes_left::<4>(),
        (true, 5) => simd.rotate_lanes_left::<5>(),
        (true, 6) => simd.rotate_lanes_left::<6>(),
        (false, 1) => simd.rotate_lanes_right::<1>(),
        (false, 2) => simd.rotate_lanes_right::<2>(),
        (false, 3) => simd.rotate_lanes_right::<3>(),
        (false, 4) => simd.rotate_lanes_right::<4>(),
        (false, 5) => simd.rotate_lanes_right::<5>(),
        (false, 6) => simd.rotate_lanes_right::<6>(),
        _ => panic!("invalid rotation, only 1-6 are supported here"),
    }
}

#[inline]
fn rotate_dyn_dyn<T: SimdElement>(simd: Simd<T, 32>, cnt: usize, left: bool) -> Simd<T, 32> {
    match (left, cnt) {
        (true, 1) => simd.rotate_lanes_left::<1>(),
        (true, 2) => simd.rotate_lanes_left::<2>(),
        (true, 3) => simd.rotate_lanes_left::<3>(),
        (true, 4) => simd.rotate_lanes_left::<4>(),
        (true, 5) => simd.rotate_lanes_left::<5>(),
        (true, 6) => simd.rotate_lanes_left::<6>(),
        (false, 1) => simd.rotate_lanes_right::<1>(),
        (false, 2) => simd.rotate_lanes_right::<2>(),
        (false, 3) => simd.rotate_lanes_right::<3>(),
        (false, 4) => simd.rotate_lanes_right::<4>(),
        (false, 5) => simd.rotate_lanes_right::<5>(),
        (false, 6) => simd.rotate_lanes_right::<6>(),
        _ => panic!("invalid rotation, only 1-6 are supported here"),
    }
}

impl MoveGen for Simd1MoveGenerator {
    fn gen_moves_one_die(
        board: &crate::types::board::Board,
        die: crate::types::dice::Die,
        player: crate::types::prim::Bw,
    ) -> Vec<Board> {
        timings::time(PerfParts::OneDie, || {
            let bar = BoardCoord::bar(player);

            if !board[bar].is_empty() {
                return timings::time(PerfParts::Bar, || {
                    let to = (bar + (player, die)).unwrap();

                    match board[to].to_bwn() {
                        Some((opp, 1)) if opp != player => {
                            let mut b = board.clone();

                            b[bar] -= 1;
                            b[to] = BOrW::from((player, 1));
                            b.inc_bar(opp);

                            return vec![b];
                        }
                        Some((opp, _)) if opp != player => return vec![],
                        Some((_, _)) => {
                            let mut b = board.clone();

                            b[bar] -= 1;
                            b[to] += 1;

                            return vec![b];
                        }
                        None => {
                            let mut b = board.clone();

                            b[bar] -= 1;
                            b[to] = BOrW::from((player, 1));

                            return vec![b];
                        }
                    };
                });
            };

            let simdboard = board.0;
            let udie = die.0 as usize;

            let mut moves = timings::time(PerfParts::Removals, || {
                let mut anti_home = Simd::splat(1);
                match player {
                    Bw::White => {
                        for i in 1..=6 {
                            anti_home[i] = 0;
                        }
                    }
                    Bw::Black => {
                        for i in 19..=24 {
                            anti_home[i] = 0;
                        }
                    }
                };
                let no_home = simdboard * anti_home;

                let removals_possible = match player {
                    Bw::White => !no_home.is_positive().any(),
                    Bw::Black => !no_home.is_negative().any(),
                };

                if !removals_possible {
                    vec![]
                } else {
                    // let range = match player {
                    //     Bw::White => 1..=udie,
                    //     Bw::Black => (18 - udie)..=18,
                    // };

                    (1..=udie as u8)
                        .filter_map(|i| {
                            let c = BoardCoord::rel(player, i);
                            if board[c].matches(player) {
                                let mut b = board.clone();

                                b[c] -= 1;

                                Some(b)
                            } else {
                                None
                            }
                        })
                        .collect()
                }
            });

            let good_targets = timings::time(PerfParts::Calc, || {
                let board_targets = rotate_dyn_dyn(simdboard.clone(), udie, player == Bw::White);

                let valid_from_source = match player {
                    Bw::White => board_targets.is_positive(),
                    Bw::Black => board_targets.is_negative(),
                };

                let targets_forbidden = match player {
                    Bw::White => simdboard.simd_lt(Simd::splat(-1)),
                    Bw::Black => simdboard.simd_gt(Simd::splat(1)),
                };

                let good_targets = valid_from_source & !targets_forbidden;

                // println!("die: {}", udie);
                // println!("\nboard: {:?}", board);
                // println!(
                //     "vfsource:         {}",
                //     debug_bool_board(&valid_from_source.to_array()[1..=24])
                // );
                // println!(
                //     "tforbidd:         {}",
                //     debug_bool_board(&targets_forbidden.to_array()[1..=24])
                // );
                // println!(
                //     "goodtarg:         {}",
                //     debug_bool_board(&good_targets.to_array()[1..=24])
                // );

                good_targets
            });

            timings::time(PerfParts::GenBoards, || {
                // can't be more than 15 moves, because checkers are 15

                for lane in 1..=24 {
                    if good_targets.test(lane) {
                        let mut b = board.clone();

                        let to = BoardCoord(lane as u8);
                        let Some(from) = to + (-player, die) else {
                            continue;
                        };

                        // println!("lane: {}", lane);
                        // println!("from: {:?}, to: {:?}", from, to);
                        // println!("from: {}, to: {}", board[from], board[to]);

                        match board[to].to_bwn() {
                            Some((opp, 1)) if opp != player => {
                                b[from] -= 1;
                                b[to] = BOrW::from((player, 1));
                                b.inc_bar(opp);

                                moves.push(b);
                            }
                            Some((opp, _)) if opp != player => {
                                unreachable!("this should not happen")
                            }
                            Some((_, _)) => {
                                b[from] -= 1;
                                b[to] += 1;

                                moves.push(b);
                            }
                            None => {
                                b[from] -= 1;
                                b[to] = BOrW::from((player, 1));

                                moves.push(b);
                            }
                        };
                    }
                }

                moves
            })
        })
    }
}

#[cfg(test)]
mod test {
    use once_cell::sync::Lazy;
    use rand::Rng;

    use crate::{
        movegen::{basic::BasicMoveGenerator, MoveGen},
        types::{board::Board, dice::Die, prim::Bw},
        util::timings::PERF_MAP,
    };

    use super::Simd1MoveGenerator;

    #[test]
    pub fn test_same_random_moves() {
        let rng = &mut rand::thread_rng();
        Lazy::force(&PERF_MAP);

        for iter in 0..1000 {
            let board: Board = rng.gen();
            let die: Die = rng.gen();
            let player: Bw = rng.gen();

            // println!("board: {:?}", board);
            // println!("die: {:?}", die);
            // println!("player: {:?}", player);

            let mut basic_moves = BasicMoveGenerator::gen_moves_one_die(&board, die, player);
            let mut simd_moves = Simd1MoveGenerator::gen_moves_one_die(&board, die, player);

            basic_moves.sort();
            simd_moves.sort();

            assert_eq!(basic_moves, simd_moves, "iter: {}", iter);
        }

        println!("{:?}", *PERF_MAP);
    }
}
