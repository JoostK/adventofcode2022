use std::ops::{BitAnd, BitOr};

#[derive(Copy, Clone)]
pub struct Bag(u64);

impl Bag {
    pub fn new(items: &str) -> Self {
        Self(
            items
                .chars()
                .fold(0u64, |bits, char| bits.bitor(1 << to_priority(char))),
        )
    }

    pub fn intersect(self, other: Bag) -> Bag {
        Self(self.0.bitand(other.0))
    }

    pub fn priority(&self) -> u8 {
        debug_assert!(self.0.is_power_of_two());
        self.0.trailing_zeros() as u8
    }
}

fn to_priority(letter: char) -> u8 {
    if letter.is_ascii_lowercase() {
        letter as u8 - b'a' + 1
    } else {
        letter as u8 - b'A' + 1 + 26
    }
}
