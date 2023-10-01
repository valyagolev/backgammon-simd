use crate::types::{
    board::{Board, BoardCoord},
    prim::{BOrW, Bw},
};

use super::MoveGen;

pub struct BasicMoveGenerator;

fn any_not_home(player: Bw, board: &Board) -> bool {
    for (coord, val) in board.iter_inner_board() {
        if val.matches(player) {
            if !coord.is_home(player) {
                return true;
            }
        }
    }

    false
}

impl MoveGen for BasicMoveGenerator {
    fn gen_moves_one_die(
        board: &crate::types::board::Board,
        die: crate::types::dice::Die,
        player: crate::types::prim::Bw,
    ) -> Vec<Board> {
        let bar = BoardCoord::bar(player);

        if !board[bar].is_empty() {
            let to = (bar + (player, die)).unwrap();

            match board[to].to_bwn() {
                Some((own, 1)) if own != player => {
                    let mut b = board.clone();

                    b[bar] -= 1;
                    b[to] = BOrW::from((player, 1));
                    b.inc_bar(own);

                    return vec![b];
                }
                Some((own, _)) if own != player => return vec![],
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
        }

        let mut moves = Vec::new();

        if !any_not_home(player, board) {
            for (coord, val) in board.iter_inner_board() {
                let Some((owner, _)) = val.to_bwn() else {
                    continue;
                };

                if owner != player {
                    continue;
                }

                if coord.perspective(player) <= die.0 {
                    let mut b = board.clone();

                    b[coord] -= 1;
                    // b[coord + (player, die)] += 1;

                    moves.push(b);
                }
            }
        }

        for (coord, val) in board.iter_inner_board() {
            if let Some((own, _)) = val.to_bwn() {
                if own != player {
                    continue;
                }

                let Some(to) = coord + (player, die) else {
                    continue;
                };

                match board[to].to_bwn() {
                    Some((own, 1)) if own != player => {
                        let mut b = board.clone();

                        b[coord] -= 1;
                        b[to] = BOrW::from((player, 1));
                        b.inc_bar(own);

                        moves.push(b);
                    }
                    Some((own, _)) if own != player => continue,
                    Some((_, _)) => {
                        let mut b = board.clone();

                        b[coord] -= 1;
                        b[to] += 1;

                        moves.push(b);
                    }
                    None => {
                        let mut b = board.clone();

                        b[coord] -= 1;
                        b[to] = BOrW::from((player, 1));

                        moves.push(b);
                    }
                };
            }
        }

        moves
    }
}
