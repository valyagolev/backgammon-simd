// use crate::movegen::MoveGen;

// pub struct BackgammonCrateCompatMoveGenerator;

// impl MoveGen for BackgammonCrateCompatMoveGenerator {
//     fn gen_moves_one_die(
//         board: &crate::types::board::Board,
//         die: crate::types::dice::Die,
//         player: crate::types::prim::Bw,
//     ) -> Vec<crate::types::board::Board> {
//         // let game = crate::game::Game {
//         //     board: board.clone(),
//         //     state: crate::game::GameState::Dice(player, die),
//         // };

//         todo!()
//     }

//     fn gen_unique_moves(
//         board: &crate::types::board::Board,
//         dice: crate::types::dice::Dice,
//         player: crate::types::prim::Bw,
//     ) -> Vec<crate::types::board::Board> {
//         let game = &crate::game::Game {
//             board: board.clone(),
//             state: crate::game::GameState::Dice(player, dice),
//         };
//         let their_game: backgammon::Game = game.try_into().unwrap();

//         todo!()
//     }
// }
