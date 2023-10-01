use std::{
    ops::{Add, Index, IndexMut, Neg, RangeInclusive},
    simd::{Simd, SimdInt},
};

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
            Bw::White => BoardCoord(25),
            Bw::Black => BoardCoord(0),
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
    /// where they should all end up
    pub fn is_home(&self, color: Bw) -> bool {
        if self.is_bar(color) {
            return false;
        }

        match color {
            Bw::Black => self.0 > 18,
            Bw::White => self.0 < 7,
        }
    }

    #[must_use]
    #[inline]
    pub fn is_bar(&self, color: Bw) -> bool {
        match color {
            Bw::White => self.0 == 25,
            Bw::Black => self.0 == 0,
        }
    }

    #[must_use]
    #[inline]
    pub fn perspective(&self, color: Bw) -> u8 {
        match color {
            Bw::White => self.0,
            Bw::Black => 25 - self.0,
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
            Bw::Black => self.0 + die.0,
            Bw::White => self.0.checked_sub(die.0)?,
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
    // Serialize,
    // Deserialize,
    PartialOrd,
    Ord,
)]
#[repr(transparent)]
// #[serde(transparent)]
/// from 1 to 24; 0 and 25 mean respective "bars"
pub struct Board(pub Simd<i8, 32>);

impl Board {
    pub fn empty() -> Self {
        // Self([BOrW::empty(); 26])
        Self(Simd::splat(0))
    }

    #[inline]
    pub fn iter_inner_board(&self) -> impl Iterator<Item = (BoardCoord, BOrW)> + '_ {
        BoardCoord::inner_board_coords().map(|coord| (coord, self[coord]))
    }

    #[inline]
    pub fn is_winner(&self, color: Bw) -> bool {
        match color {
            Bw::White => !self.0.is_positive().any(),
            Bw::Black => !self.0.is_negative().any(),
        }
    }

    #[inline]
    pub fn winner(&self) -> Option<Bw> {
        if self.is_winner(Bw::White) {
            Some(Bw::White)
        } else if self.is_winner(Bw::Black) {
            Some(Bw::Black)
        } else {
            None
        }
    }

    #[inline]
    pub fn inc_bar(&mut self, color: Bw) {
        self.0[BoardCoord::bar(color).0 as usize] += match color {
            Bw::White => 1,
            Bw::Black => -1,
        };
    }

    #[inline]
    pub fn dec_bar(&mut self, color: Bw) {
        self[BoardCoord::bar(color)] -= match color {
            Bw::White => 1,
            Bw::Black => -1,
        };
    }

    pub fn inverse(&self) -> Board {
        let mut arr = [0; 32];

        let sl = self.0[0..26].iter().map(|x| -x).rev().collect_vec();

        arr[0..26].copy_from_slice(&sl);

        Board(Simd::from(arr))
    }
}

impl Default for Board {
    fn default() -> Self {
        [
            (BoardCoord(1), BOrW::black(2)),
            (BoardCoord(6), BOrW::white(5)),
            (BoardCoord(8), BOrW::white(3)),
            (BoardCoord(12), BOrW::black(5)),
            (BoardCoord(13), BOrW::white(5)),
            (BoardCoord(17), BOrW::black(3)),
            (BoardCoord(19), BOrW::black(5)),
            (BoardCoord(24), BOrW::white(2)),
        ]
        .into_iter()
        .collect()
    }
}

fn print_board_chunk(chunk: &[BOrW], reverse: bool) -> String {
    let mut it = chunk.iter().map(|b| b.to_string());

    if reverse {
        it.rev().join(" ")
    } else {
        it.join(" ")
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} | {} | {} |\n   | {} | {} | {}\n",
            BOrW(self.0[0]),
            print_board_chunk(&self[BoardCoord(7)..=BoardCoord(12)], true),
            print_board_chunk(&self[BoardCoord(1)..=BoardCoord(6)], true),
            print_board_chunk(&self[BoardCoord(13)..=BoardCoord(18)], false),
            print_board_chunk(&self[BoardCoord(19)..=BoardCoord(24)], false),
            BOrW(self.0[25])
        ))
    }
}

pub fn debug_bool_board(vals: &[bool]) -> String {
    vals.iter()
        .map(|v| if *v { " *" } else { " -" })
        .chunks(6)
        .into_iter()
        .map(|mut ch| ch.join(", "))
        .join(" | ")
}

impl FromIterator<(BoardCoord, BOrW)> for Board {
    fn from_iter<T: IntoIterator<Item = (BoardCoord, BOrW)>>(iter: T) -> Self {
        let mut board = [0; 32];

        for (coord, piece) in iter {
            board[coord.0 as usize] = piece.0;
        }

        Self(Simd::from(board))
        // board)
    }
}

impl Index<BoardCoord> for Board {
    type Output = BOrW;

    #[inline]
    fn index(&self, index: BoardCoord) -> &Self::Output {
        // Safety: Transmuting a reference to i8 to a reference to BOrW
        // since BOrW is a repr(transparent) wrapper around i8
        unsafe { &*(self.0.index(index.0 as usize) as *const i8 as *const BOrW) }
    }
}

impl Index<RangeInclusive<BoardCoord>> for Board {
    type Output = [BOrW];

    fn index(&self, index: RangeInclusive<BoardCoord>) -> &Self::Output {
        // Safety: Transmuting a reference to i8 to a reference to BOrW
        // since BOrW is a repr(transparent) wrapper around i8

        let start = index.start().0 as usize;
        let end = index.end().0 as usize;
        let slice = &self.0[start..=end];

        unsafe { &*(slice as *const [i8] as *const [BOrW]) }
    }
}

impl IndexMut<BoardCoord> for Board {
    #[inline]
    fn index_mut(&mut self, index: BoardCoord) -> &mut Self::Output {
        // Safety: Transmuting a reference to i8 to a reference to BOrW
        // since BOrW is a repr(transparent) wrapper around i8
        unsafe { &mut *(self.0.index_mut(index.0 as usize) as *mut i8 as *mut BOrW) }
    }
}
