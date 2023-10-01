use std::ops::{Add, Index, IndexMut, Neg};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::{
    dice::Die,
    prim::{BOrW, Bw},
};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
// #[repr(transparent)]
#[serde(transparent)]
/// from 1 to 24; 0 and 25 mean respective "bars"
pub struct BoardCoord(pub u8);

impl BoardCoord {
    #[inline]
    pub fn inner_board_coords() -> impl Iterator<Item = BoardCoord> {
        (1..=24).map(BoardCoord)
    }

    #[inline]
    pub fn bar(color: Bw) -> BoardCoord {
        match color {
            Bw::White => BoardCoord(0),
            Bw::Black => BoardCoord(25),
        }
    }

    #[must_use]
    #[inline]
    pub fn rel(color: Bw, i: u8) -> BoardCoord {
        match color {
            Bw::White => BoardCoord(i),
            Bw::Black => -BoardCoord(i),
        }
    }

    #[must_use]
    #[inline]
    pub fn is_home(&self, color: Bw) -> bool {
        if self.is_bar(color) {
            return false;
        }

        match color {
            Bw::White => self.0 > 18,
            Bw::Black => self.0 < 7,
        }
    }

    #[must_use]
    #[inline]
    pub fn is_bar(&self, color: Bw) -> bool {
        match color {
            Bw::White => self.0 == 0,
            Bw::Black => self.0 == 25,
        }
    }
}

impl Neg for BoardCoord {
    type Output = BoardCoord;

    #[inline]
    fn neg(self) -> Self::Output {
        BoardCoord(25 - self.0)
    }
}

impl Add<(Bw, Die)> for BoardCoord {
    type Output = Option<BoardCoord>;

    #[inline]
    fn add(self, (color, die): (Bw, Die)) -> Self::Output {
        let out = match color {
            Bw::White => self.0 + die.0,
            Bw::Black => self.0.checked_sub(die.0)?,
        };

        if out <= 0 || out >= 25 {
            None
        } else {
            Some(BoardCoord(out))
        }
    }
}

#[derive(
    Hash,
    Eq,
    PartialEq,
    // Copy,
    Clone,
    Serialize,
    Deserialize,
)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Board(pub [BOrW; 26]);

impl Board {
    pub fn empty() -> Self {
        Self([BOrW::empty(); 26])
    }

    #[inline]
    pub fn iter_inner_board(&self) -> impl Iterator<Item = (BoardCoord, BOrW)> + '_ {
        BoardCoord::inner_board_coords().map(|coord| (coord, self[coord]))
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Board")
            .field(&self.0[0])
            .field(&self.0[1..=24].to_vec())
            .field(&self.0[25])
            .finish()
    }
}

pub fn debug_bool_board(vals: &[bool]) -> String {
    vals.iter().map(|v| if *v { " *" } else { " -" }).join(", ")
}

impl FromIterator<(BoardCoord, BOrW)> for Board {
    fn from_iter<T: IntoIterator<Item = (BoardCoord, BOrW)>>(iter: T) -> Self {
        let mut board = [BOrW::empty(); 26];

        for (coord, piece) in iter {
            board[coord.0 as usize] = piece;
        }

        Self(board)
    }
}

impl Index<BoardCoord> for Board {
    type Output = BOrW;

    #[inline]
    fn index(&self, index: BoardCoord) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl IndexMut<BoardCoord> for Board {
    #[inline]
    fn index_mut(&mut self, index: BoardCoord) -> &mut Self::Output {
        &mut self.0[index.0 as usize]
    }
}
