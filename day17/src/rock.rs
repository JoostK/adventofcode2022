use std::fmt::{Debug, Display, Formatter};

pub const ROCKS: [Rock; 5] = [
    // ####
    Rock::new().dot(0, 0).dot(0, 1).dot(0, 2).dot(0, 3),
    // .#.
    // ###
    // .#.
    Rock::new()
        .dot(2, 1)
        .dot(1, 0)
        .dot(1, 1)
        .dot(1, 2)
        .dot(0, 1),
    // ..#
    // ..#
    // ###
    Rock::new()
        .dot(2, 2)
        .dot(1, 2)
        .dot(0, 0)
        .dot(0, 1)
        .dot(0, 2),
    // #
    // #
    // #
    // #
    Rock::new().dot(3, 0).dot(2, 0).dot(1, 0).dot(0, 0),
    // ##
    // ##
    Rock::new().dot(1, 0).dot(1, 1).dot(0, 0).dot(0, 1),
];

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rock(pub u32);

impl Display for Rock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut b = self.0;
        let mut lines: Vec<String> = Vec::new();
        let mut line = String::new();
        while lines.len() != 4 {
            if line.len() == 7 {
                lines.push(line.chars().rev().collect());
                line = String::new();
                b >>= 1;
                continue;
            }
            if b & 1 == 1 {
                line.push('#');
            } else {
                line.push('.');
            }
            b >>= 1;
        }
        writeln!(f)?;
        for line in lines.iter().rev() {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl Debug for Rock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// Represents a rock where the y-axis points up (so line=0 => bottom of the rock) and the x-axis
/// points right (so column=0 => left of the rock). This is encoded in a u32 where the low byte
/// is the bottom line, with the most significant bit correspond with the right side.
impl Rock {
    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn dot(self, line: u32, column: u32) -> Self {
        const START_OFFSET: u32 = 2;
        // 76543210
        // |.......|
        //
        // column 0 -> bit 4
        // column 1 -> bit 3
        // column 2 -> bit 2
        // column 3 -> bit 1
        let bit = 1 << (7 - column - START_OFFSET - 1);
        Self(self.0 | (bit << (line * 8)))
    }

    pub fn left(&self) -> Option<Self> {
        let bits = self.0 << 1;
        (bits & BOUNDARY == 0).then_some(Self(bits))
    }

    pub fn right(&self) -> Option<Self> {
        if self.0 & 1 == 1 {
            // Would shift outside boundary
            return None;
        }
        let bits = self.0 >> 1;
        (bits & BOUNDARY == 0).then_some(Self(bits))
    }

    pub fn covers(&self, other: u32) -> bool {
        self.0 & other != 0
    }
}

const BOUNDARY: u32 = (1 << 7) | (1 << 15) | (1 << 23) | (1 << 31);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn left() {
        for rock in ROCKS {
            println!("{rock}");
            println!("{:?}", rock.left());
            println!("{:?}", rock.left().and_then(|s| s.left()));
            println!(
                "{:?}",
                rock.left().and_then(|s| s.left()).and_then(|s| s.left())
            );
            println!("{:?}", rock.right());
            println!("{:?}", rock.right().and_then(|s| s.right()));
            println!(
                "{:?}",
                rock.right().and_then(|s| s.right()).and_then(|s| s.right())
            );
            println!();
            println!();
            println!();
        }
    }
}
