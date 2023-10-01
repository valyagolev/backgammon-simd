use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Die(pub u8);

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Dice(pub Die, pub Die);

impl Dice {
    pub fn iter_all_possible() -> impl Iterator<Item = Dice> {
        (1..=6).flat_map(|a| (1..=6).map(move |b| Dice(Die(a), Die(b))))
    }

    #[must_use]
    #[inline]
    pub fn dice(&self) -> Vec<Die> {
        if self.0 == self.1 {
            return vec![self.0; 4];
        }

        vec![self.0, self.1]
    }

    #[inline]
    pub fn roll(rng: &mut impl Rng) -> Dice {
        rng.gen()
    }

    #[inline]
    pub fn as_u8(&self) -> u8 {
        self.0 .0 << 3 | self.1 .0
    }

    #[inline]
    pub fn from_u8(u: u8) -> Dice {
        Dice(Die(u >> 3), Die(u & 0b111))
    }

    #[inline]
    pub fn as_alnum(&self) -> char {
        let index = (self.0 .0 - 1) * 6 + (self.1 .0 - 1);
        match index {
            0..=9 => ('0' as u8 + index) as char,
            10..=35 => ('A' as u8 + index - 10) as char,
            _ => panic!("Invalid dice rolls"),
        }
    }

    #[inline]
    pub fn from_alnum(c: char) -> Self {
        let index = match c {
            '0'..='9' => c as u8 - '0' as u8,
            'A'..='Z' => 10 + (c as u8 - 'A' as u8),
            _ => panic!("Invalid character"),
        };
        let first = index / 6 + 1;
        let second = index % 6 + 1;
        Dice(Die(first), Die(second))
    }

    #[inline]
    pub fn is_double(&self) -> bool {
        self.0 == self.1
    }
}

impl Distribution<Die> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Die {
        Die(rng.gen_range(1..=6))
    }
}

impl Distribution<Dice> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Dice {
        Dice(rng.gen(), rng.gen())
    }
}
