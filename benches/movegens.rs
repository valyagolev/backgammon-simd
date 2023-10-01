use std::time::Duration;

use backgammon_simd::movegen::simd::Simd1MoveGenerator;
use backgammon_simd::movegen::MoveGen;
use backgammon_simd::util::timings::PERF_MAP;
use backgammon_simd::{
    movegen::basic::BasicMoveGenerator,
    types::{board::Board, dice::Dice, prim::Bw},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use once_cell::sync::Lazy;
use rand::Rng;

pub fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("moves for initial pos", |b| {
    //     b.iter(|| {
    //         let mut board = BgGame::new();
    //         board
    //             .apply_event(&mut rand::thread_rng(), GameEvent::RollDice)
    //             .unwrap();

    //         let (player, dice) = match board.status {
    //             BgGameState::Dice(player, dice) => (player, dice),
    //             _ => unreachable!(),
    //         };

    //         let _ = black_box(Move::all_moves(&board, dice, player));
    //     })
    // });

    // let mut c = c.measurement_time(Duration::from_secs(10));

    Lazy::force(&PERF_MAP);

    c.bench_function("Simd1MoveGenerator moves for random pos (x100)", |b| {
        b.iter_batched_ref(
            || {
                let rng = &mut rand::thread_rng();
                (0..100)
                    .map(|_| (rng.gen(), rng.gen(), rng.gen()))
                    .collect::<Vec<(Board, Dice, Bw)>>()
            },
            |inp| {
                for (board, dice, player) in inp {
                    // println!("board: {:?}", board);
                    let _ = black_box(Simd1MoveGenerator::gen_unique_moves(board, *dice, *player));
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });

    println!("{:?}", *PERF_MAP);

    c.bench_function("BasicMoveGenerator moves for random pos (x100)", |b| {
        b.iter_batched_ref(
            || {
                let rng = &mut rand::thread_rng();
                (0..100)
                    .map(|_| (rng.gen(), rng.gen(), rng.gen()))
                    .collect::<Vec<(Board, Dice, Bw)>>()
            },
            |inp| {
                for (board, dice, player) in inp {
                    // println!("board: {:?}", board);
                    let _ = black_box(BasicMoveGenerator::gen_unique_moves(board, *dice, *player));
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = criterion_benchmark
}
criterion_main!(benches);
