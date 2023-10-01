use std::marker::PhantomData;

use rand::Rng;

use crate::{
    decision::MoveDecision,
    movegen::{simd::Simd1MoveGenerator, MoveGen},
    types::{board::Board, dice::Dice, prim::Bw},
};

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub state: GameState,
}

impl Game {
    pub fn new(rng: &mut impl Rng) -> Self {
        let mut dice = Dice::roll(rng);
        while dice.0 == dice.1 {
            dice = Dice::roll(rng);
        }

        let player = if dice.0 > dice.1 {
            Bw::White
        } else {
            Bw::Black
        };

        Self {
            board: Board::default(),
            state: GameState::Dice(player, dice),
        }
    }

    pub fn next_moves<Generator: MoveGen>(&self) -> Vec<Board> {
        match self.state {
            GameState::Dice(player, dice) => {
                Generator::gen_unique_moves(&self.board, dice, player)
                // Simd1MoveGenerator::gen_unique_moves(&self.board, dice, player)
                // BasicMoveGenerator::gen_unique_moves(&self.board, dice, player)
            }
            GameState::Finished(_) => panic!("cannot get moves on finished game"),
        }
    }

    pub fn make_move_unchecked(&mut self, rng: &mut impl Rng, new_board: Board) {
        let GameState::Dice(player, _) = self.state else {
            panic!("cannot make move on finished game");
        };

        self.board = new_board;

        if let Some(player) = self.board.winner() {
            self.state = GameState::Finished(player);
        } else {
            let dice = Dice::roll(rng);
            self.state = GameState::Dice(-player, dice);
        }
    }

    pub fn skip_move_by_necessity(&mut self, rng: &mut impl Rng) {
        let GameState::Dice(player, _) = self.state else {
            panic!("cannot skip move on finished game");
        };

        // if let Some(player) = self.board.winner() {
        //     self.state = GameState::Finished(player);
        // } else {

        let dice = Dice::roll(rng);
        self.state = GameState::Dice(-player, dice);
    }

    pub fn play<'a, R: Rng, White: MoveDecision, Black: MoveDecision>(
        &'a mut self,
        rng: R,
        white: &'a mut White,
        black: &'a mut Black,
    ) -> GamePlay<'a, R, White, Black> {
        GamePlay {
            game: self,
            rng,
            white,
            black: Some(black),
            generator: PhantomData::<Simd1MoveGenerator>,
        }
    }

    pub fn play_self<'a, R: Rng, White: MoveDecision>(
        &'a mut self,
        rng: R,
        white: &'a mut White,
    ) -> GamePlay<'a, R, White> {
        GamePlay {
            game: self,
            rng,
            white,
            black: None,
            generator: PhantomData::<Simd1MoveGenerator>,
        }
    }

    pub fn is_finished(&self) -> bool {
        match self.state {
            GameState::Finished(_) => true,
            GameState::Dice(_, _) => false,
        }
    }
}

pub struct GamePlay<
    'a,
    R: Rng,
    White: MoveDecision,
    Black: MoveDecision = White,
    Generator: MoveGen = Simd1MoveGenerator,
> {
    game: &'a mut Game,
    rng: R,
    white: &'a mut White,
    generator: PhantomData<Generator>,
    black: Option<&'a mut Black>,
}

impl<'a, R: Rng, White: MoveDecision, Black: MoveDecision, Generator: MoveGen> Iterator
    for GamePlay<'a, R, White, Black, Generator>
{
    type Item = Game;

    fn next(&mut self) -> Option<Self::Item> {
        match self.game.state {
            GameState::Finished(_) => None,
            GameState::Dice(player, _) => {
                let moves = self.game.next_moves::<Generator>();

                if moves.is_empty() {
                    self.game.skip_move_by_necessity(&mut self.rng);
                } else {
                    let mov = match (&mut self.black, player) {
                        (None, _) => self.white.choose(player, moves),
                        (_, Bw::White) => self.white.choose(player, moves),
                        (Some(black), _) => black.choose(player, moves),
                    };
                    self.game.make_move_unchecked(&mut self.rng, mov.clone());
                }

                Some(self.game.clone())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum GameState {
    Dice(Bw, Dice),
    Finished(Bw),
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameState::Dice(bw, dice) => write!(f, "{}: {} {}", bw, dice.0 .0, dice.1 .0),
            GameState::Finished(bw) => write!(f, "{} won", bw),
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}\n{}", self.board, self.state)
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use crate::{
        decision::RandomMoveDecision,
        game::{Game, GameState},
        movegen::{simd::Simd1MoveGenerator, MoveGen},
        types::prim::Bw,
    };

    #[test]
    fn print_game() {
        let mut game = Game::new(&mut rand::thread_rng());
        // game.state = GameState::Dice(Bw::White, Dice(Die(1), Die(3)));

        println!("{}", game);

        for game in game
            .play(
                rand::thread_rng(),
                &mut RandomMoveDecision(rand::thread_rng()),
                &mut RandomMoveDecision(rand::thread_rng()),
            )
            .take(100)
        {
            println!("{}\n", game);
        }
    }

    #[test]
    fn count_game() {
        let mut game = Game::new(&mut rand::thread_rng());

        println!(
            "{}",
            game.play(
                rand::thread_rng(),
                &mut RandomMoveDecision(rand::thread_rng()),
                &mut RandomMoveDecision(rand::thread_rng()),
            )
            .count()
        )
    }

    #[test]
    fn random_games_stats() {
        let mut blacks = 0;
        let mut whites = 0;

        for _ in 0..100 {
            let mut game = Game::new(&mut rand::thread_rng());

            println!(
                "{}",
                game.play(
                    rand::thread_rng(),
                    &mut RandomMoveDecision(rand::thread_rng()),
                    &mut RandomMoveDecision(rand::thread_rng()),
                )
                .count()
            );

            match game.state {
                GameState::Dice(_, _) => {}
                GameState::Finished(Bw::Black) => {
                    blacks += 1;
                }
                GameState::Finished(Bw::White) => {
                    whites += 1;
                }
            }
        }

        println!("{} / {}", whites, blacks);
    }

    #[test]
    pub fn test_generators_same() {
        for _ in 0..100 {
            let mut game = Game::new(&mut rand::thread_rng());
            let a = &mut RandomMoveDecision(rand::thread_rng());
            let b = &mut RandomMoveDecision(rand::thread_rng());

            let play = game.play(rand::thread_rng(), a, b);

            for game in play {
                let GameState::Dice(player, dice) = game.state else {
                    continue;
                };

                // assert_eq!(game.board, game.board.inverse());

                let mut normal = Simd1MoveGenerator::gen_unique_moves(&game.board, dice, player);
                let mut reversed =
                    Simd1MoveGenerator::gen_unique_moves(&game.board.inverse(), dice, -player)
                        .into_iter()
                        .map(|b| b.inverse())
                        .collect_vec();

                normal.sort();
                reversed.sort();

                // if normal != reversed {}
                if normal.len() <= 2 {
                    if normal != reversed {
                        println!("{}", game);
                        println!("{} {}", normal.len(), reversed.len());

                        println!("MOVES BY NORMAL:");

                        for brd in normal {
                            println!("{brd:?}");
                        }

                        println!("MOVES BY REVERSED:");

                        for brd in reversed {
                            println!("{brd:?}");
                        }

                        panic!();
                    }
                }

                // let mut s = game.next_moves::<Simd1MoveGenerator>();
                // s.sort();
                // let mut b = game.next_moves::<BasicMoveGenerator>();
                // b.sort();

                // if s != b && s.len() < 3 {
                //     println!("{}", game);
                //     println!("{} {}", s.len(), b.len());

                //     println!("MOVES BY BASIC:");

                //     for brd in b {
                //         println!("{brd:?}");
                //     }

                //     println!("MOVES BY SIMD:");
                //     for brd in s {
                //         println!("{brd:?}");
                //     }

                //     panic!();
                // }
            }
        }
    }
}
