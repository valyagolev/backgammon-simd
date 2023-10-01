use backgammon::{self};

use crate::{
    game::GameState,
    types::{board::BoardCoord, prim::Bw},
};

pub mod movegen;
pub mod tests;

impl Into<backgammon::rules::Player> for Bw {
    fn into(self) -> backgammon::rules::Player {
        match self {
            Bw::White => backgammon::rules::Player::Player0,
            Bw::Black => backgammon::rules::Player::Player1,
        }
    }
}

impl TryFrom<backgammon::rules::Player> for Bw {
    type Error = anyhow::Error;

    fn try_from(value: backgammon::rules::Player) -> anyhow::Result<Self> {
        match value {
            backgammon::rules::Player::Player0 => Ok(Bw::White),
            backgammon::rules::Player::Player1 => Ok(Bw::Black),
            _ => Err(anyhow::anyhow!("invalid player")),
        }
    }
}

impl Into<backgammon::rules::Dices> for crate::types::dice::Dice {
    fn into(self) -> backgammon::rules::Dices {
        backgammon::rules::Dices {
            values: (self.0 .0, self.1 .0),
            consumed: (false, false, false, false),
        }
    }
}

impl Into<backgammon::rules::Board> for &crate::types::board::Board {
    fn into(self) -> backgammon::rules::Board {
        let [bw, bb] = [Bw::White, Bw::Black].map(|bw| {
            let mut total = 0;

            let mut own = (0..=25)
                .map(|c| {
                    let borw = self[BoardCoord(c)];

                    if borw.matches(bw) {
                        total += borw.to_count();
                        borw.to_count()
                    } else {
                        0
                    }
                })
                .collect::<Vec<_>>();

            if bw == Bw::Black {
                own.reverse();
            }

            assert!(total <= 15);

            let bar = own[BoardCoord::bar(bw).0 as usize];
            let off = 15 - total;

            backgammon::rules::PlayerBoard {
                board: own[1..25].try_into().unwrap(),
                bar,
                off,
            }
        });

        backgammon::rules::Board {
            raw_board: (bw, bb),
        }
    }
}

impl TryInto<backgammon::Game> for &crate::game::Game {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<backgammon::Game> {
        let GameState::Dice(player, dice) = self.state else {
            return Err(anyhow::anyhow!("game is not in dice state"));
        };
        let mut game = backgammon::Game::new();

        // game.rules = Rules::default();
        game.dices = dice.into();
        game.who_plays = player.into();
        game.board = (&self.board).into();
        // game.cube = todo!();

        Ok(game)
    }
}
// impl From<backgammon::Game> for crate::game::Game {}
