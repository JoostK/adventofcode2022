use crate::rock::Rock;
use std::fmt::{Display, Formatter, Write};

pub struct Cave {
    pub lines: Vec<u8>,
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for &line in self.lines.iter().skip(1).rev() {
            let mut b = 1 << 6;
            f.write_char('|')?;
            while b != 0 {
                if line & b == b {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
                b >>= 1;
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+-------+")?;
        Ok(())
    }
}

impl Default for Cave {
    fn default() -> Self {
        Self {
            lines: vec![(1 << 7) - 1],
        }
    }
}

impl Cave {
    pub fn depth(&self) -> usize {
        // Subtract one to account for the floor
        self.lines.len() - 1
    }

    pub fn drop_rock(
        &mut self,
        mut shape: Rock,
        jetstream: &mut impl Iterator<Item = (usize, u8)>,
    ) {
        shape = shift(shape, jetstream.next().unwrap().1);
        shape = shift(shape, jetstream.next().unwrap().1);
        shape = shift(shape, jetstream.next().unwrap().1);
        shape = shift(shape, jetstream.next().unwrap().1);

        for (grid, level) in self.fall() {
            if shape.covers(grid) {
                self.record(level, shape);
                break;
            }

            let shifted = shift(shape, jetstream.next().unwrap().1);
            if !shifted.covers(grid) {
                shape = shifted;
            }
        }
    }

    fn fall(&self) -> Fall<'_> {
        Fall {
            cave: self,
            current: 0,
            level: self.lines.len(),
        }
    }

    fn record(&mut self, mut level: usize, shape: Rock) {
        let mut b = shape.0;
        while b > 0 {
            if level >= self.lines.len() {
                self.lines.push(b as u8);
            } else {
                self.lines[level] |= b as u8;
            }
            level += 1;
            b >>= 8;
        }
    }
}

pub struct Fall<'a> {
    cave: &'a Cave,
    current: u32,
    level: usize,
}

impl Iterator for Fall<'_> {
    type Item = (u32, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.level == 0 {
            return None;
        }
        self.level -= 1;

        self.current <<= 8;
        self.current |= self.cave.lines[self.level] as u32;

        Some((self.current, self.level + 1))
    }
}

pub fn shift(shape: Rock, jet: u8) -> Rock {
    debug_assert!(jet == b'<' || jet == b'>');
    if jet == b'<' {
        shape.left().unwrap_or(shape)
    } else {
        shape.right().unwrap_or(shape)
    }
}
