use std::{
    fmt::{Display, Formatter},
    ops::{AddAssign, Neg, SubAssign},
};

// use colored::Colorize;

use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Bw {
    Black = 0,
    White = 1,
}

impl Bw {
    #[inline]
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    #[inline]
    pub fn as_alnum(&self) -> char {
        match self {
            Bw::Black => 'B',
            Bw::White => 'W',
        }
    }
}

impl Display for Bw {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Bw::Black => write!(f, "Black"),
            Bw::White => write!(f, "White"),
        }
    }
}

impl Neg for Bw {
    type Output = Bw;

    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Bw::Black => Bw::White,
            Bw::White => Bw::Black,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct BOrW(pub i8);

impl Display for BOrW {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.to_bwn() {
            Some((bw, n)) => write!(f, "{}{}", bw.as_alnum(), n),
            None => write!(f, "__"),
        }
    }
}

impl std::fmt::Debug for BOrW {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.to_bwn() {
            Some((bw, n)) => f.write_str(&format!("{}{}", bw.as_alnum(), n)),
            /*.color(match bw {
                Bw::Black => "red",
                Bw::White => "blue",
            })) */
            None => f.write_str(" 0"),
        }
    }
}

impl BOrW {
    #[inline]
    pub const fn empty() -> BOrW {
        BOrW(0)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn to_bwn(self) -> Option<(Bw, u8)> {
        if self.0 > 0 {
            Some((Bw::White, self.0 as u8))
        } else if self.0 < 0 {
            Some((Bw::Black, -self.0 as u8))
        } else {
            None
        }
    }

    #[inline]
    pub fn to_count(self) -> u8 {
        self.0.abs() as u8
    }

    #[inline]
    pub fn to_color(self) -> Option<Bw> {
        if self.0 > 0 {
            Some(Bw::White)
        } else if self.0 < 0 {
            Some(Bw::Black)
        } else {
            None
        }
    }

    #[inline]
    pub fn matches(&self, bw: Bw) -> bool {
        self.to_color() == Some(bw)
    }

    #[inline]
    pub fn white(n: u8) -> Self {
        Self(n as i8)
    }

    #[inline]
    pub fn black(n: u8) -> Self {
        Self(-(n as i8))
    }

    #[inline]
    pub fn as_u32(self) -> u32 {
        self.0 as u32
    }

    #[inline]
    pub fn as_alnum(self) -> char {
        match self.to_bwn() {
            Some((Bw::White, n)) => (n + b'a') as char,
            Some((Bw::Black, n)) => (n + b'A') as char,
            None => '0',
        }
    }
}

impl Default for BOrW {
    #[inline]
    fn default() -> Self {
        Self(0)
    }
}

impl From<(Bw, u8)> for BOrW {
    #[inline]
    fn from((bw, n): (Bw, u8)) -> Self {
        Self(match (n, bw) {
            (0, _) => 0,
            (_, Bw::Black) => -(n as i8),
            (_, Bw::White) => n as i8,
        })
    }
}

impl AddAssign<i8> for BOrW {
    #[inline]
    fn add_assign(&mut self, mut rhs: i8) {
        if self.0 < 0 {
            rhs = -rhs;
        }

        self.0 += rhs;
    }
}

impl SubAssign<i8> for BOrW {
    #[inline]
    fn sub_assign(&mut self, rhs: i8) {
        debug_assert!(self.0 != 0);

        *self += -rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct BAndW([u8; 2]);

impl BAndW {
    #[inline]
    pub fn new(black: u8, white: u8) -> Self {
        Self([black, white])
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0[0] == 0 && self.0[1] == 0
    }

    #[inline]
    pub fn get(&self, bw: &Bw) -> u8 {
        self.0[*bw as usize]
    }

    #[inline]
    pub fn set(&mut self, bw: &Bw, n: u8) {
        self.0[*bw as usize] = n
    }

    #[inline]
    pub fn empty() -> Self {
        Self([0, 0])
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        (self.0[0] as u32) << 8 | self.0[1] as u32
    }

    #[inline]
    pub fn from_u32(n: u32) -> Self {
        Self([(n >> 8) as u8, n as u8])
    }

    #[inline]
    pub fn as_alnum(&self) -> [char; 2] {
        [(self.0[0] + b'A') as char, (self.0[1] + b'a') as char]
    }
}
