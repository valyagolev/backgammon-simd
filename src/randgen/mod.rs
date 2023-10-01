use once_cell::sync::Lazy;

use rand::{distributions::Standard, prelude::Distribution};
use rand_distr::WeightedAliasIndex;

use crate::types::{
    board::{Board, BoardCoord},
    prim::Bw,
};

static WHICH_HAS_EATEN: Lazy<WeightedAliasIndex<f64>> =
    Lazy::new(|| WeightedAliasIndex::new(vec![0.7, 0.14, 0.14, 0.02]).unwrap());

static EATEN_PROBS: Lazy<WeightedAliasIndex<f64>> =
    Lazy::new(|| WeightedAliasIndex::new(vec![0.15, 0.04, 0.01]).unwrap());

static GAME_STAGE: Lazy<WeightedAliasIndex<f64>> =
    Lazy::new(|| WeightedAliasIndex::new(vec![0.8, 0.2]).unwrap());

// added a bit to bloat because we allow double-columning
static STACK_SIZE: Lazy<WeightedAliasIndex<f64>> =
    Lazy::new(|| WeightedAliasIndex::new(vec![0.1 + 0.15, 0.45, 0.25, 0.1, 0.05]).unwrap());

impl Distribution<Bw> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Bw {
        if rng.gen() {
            Bw::White
        } else {
            Bw::Black
        }
    }
}

impl Distribution<BoardCoord> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> BoardCoord {
        BoardCoord(rng.gen_range(1..=24))
    }
}

fn put_on_board<R: rand::Rng + ?Sized>(
    rng: &mut R,
    board: &mut Board,
    bw: Bw,
    remaining: &mut usize,
) {
    let (pos, cur) = loop {
        let pos = rng.gen::<BoardCoord>();

        if board[pos].is_empty() {
            break (pos, 0);
        }
        if board[pos].matches(bw) {
            break (pos, board[pos].to_count());
        }
    };

    let cnt = (*remaining).min(STACK_SIZE.sample(rng) + 1);

    board[pos] = (bw, cur + cnt as u8).into();

    *remaining -= cnt;
}

impl Distribution<Board> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Board {
        let which_eaten = WHICH_HAS_EATEN.sample(rng);

        let (white_eaten, black_eaten) = match which_eaten {
            0 => (0, 0),
            1 => (EATEN_PROBS.sample(rng), 0),
            2 => (0, EATEN_PROBS.sample(rng)),
            3 => (EATEN_PROBS.sample(rng), EATEN_PROBS.sample(rng)),
            _ => unreachable!(),
        };

        let game_stage_index = GAME_STAGE.sample(rng);

        let (white_removed, black_removed) = match game_stage_index {
            0 => (0, 0),
            1 => (rng.gen_range(0..14), rng.gen_range(0..14)),
            _ => unreachable!(),
        };

        let mut white_remaining = 15 - white_eaten - white_removed;
        let mut black_remaining = 15 - black_eaten - black_removed;

        let mut board = Board::empty();

        while white_remaining > 0 && black_remaining > 0 {
            if white_remaining > 0 {
                put_on_board(rng, &mut board, Bw::White, &mut white_remaining);
            }
            if black_remaining > 0 {
                put_on_board(rng, &mut board, Bw::Black, &mut black_remaining);
            }
        }

        board[BoardCoord::bar(Bw::White)] = (Bw::White, white_eaten as u8).into();
        board[BoardCoord::bar(Bw::Black)] = (Bw::Black, black_eaten as u8).into();

        board
    }
}
