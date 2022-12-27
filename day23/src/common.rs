use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub struct Grid {
    pub elves: HashSet<(isize, isize)>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut elves = HashSet::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.bytes().enumerate() {
                if c == b'#' {
                    elves.insert((x as isize, y as isize));
                }
            }
        }

        Ok(Self { elves })
    }
}

impl Grid {
    pub fn simulate(&mut self, round: usize) -> bool {
        let first = round % 4;
        let mut proposals: HashMap<(isize, isize), Proposal> =
            HashMap::with_capacity(self.elves.len());
        for &(x, y) in &self.elves {
            let neighbours = self.neighbours(x, y);

            if neighbours != 0 {
                let north = neighbours & 0b1000_0011 != 0;
                let east = neighbours & 0b0000_1110 != 0;
                let south = neighbours & 0b0011_1000 != 0;
                let west = neighbours & 0b1110_0000 != 0;

                match (first, north, south, west, east) {
                    (0, false, _, _, _) => proposals.entry((x, y - 1)).or_default().insert(x, y),
                    (0, _, false, _, _) => proposals.entry((x, y + 1)).or_default().insert(x, y),
                    (0, _, _, false, _) => proposals.entry((x - 1, y)).or_default().insert(x, y),
                    (0, _, _, _, false) => proposals.entry((x + 1, y)).or_default().insert(x, y),
                    //
                    (1, _, false, _, _) => proposals.entry((x, y + 1)).or_default().insert(x, y),
                    (1, _, _, false, _) => proposals.entry((x - 1, y)).or_default().insert(x, y),
                    (1, _, _, _, false) => proposals.entry((x + 1, y)).or_default().insert(x, y),
                    (1, false, _, _, _) => proposals.entry((x, y - 1)).or_default().insert(x, y),
                    //
                    (2, _, _, false, _) => proposals.entry((x - 1, y)).or_default().insert(x, y),
                    (2, _, _, _, false) => proposals.entry((x + 1, y)).or_default().insert(x, y),
                    (2, false, _, _, _) => proposals.entry((x, y - 1)).or_default().insert(x, y),
                    (2, _, false, _, _) => proposals.entry((x, y + 1)).or_default().insert(x, y),
                    //
                    (3, _, _, _, false) => proposals.entry((x + 1, y)).or_default().insert(x, y),
                    (3, false, _, _, _) => proposals.entry((x, y - 1)).or_default().insert(x, y),
                    (3, _, false, _, _) => proposals.entry((x, y + 1)).or_default().insert(x, y),
                    (3, _, _, false, _) => proposals.entry((x - 1, y)).or_default().insert(x, y),
                    //
                    _ => (),
                }
            }
        }

        for (pos, proposal) in &proposals {
            if let Proposal::Elf(x, y) = *proposal {
                self.elves.insert(*pos);
                self.elves.remove(&(x, y));
            }
        }

        proposals.is_empty()
    }

    fn neighbours(&self, x: isize, y: isize) -> u8 {
        (self.elves.contains(&(x, y - 1)) as u8)
            | (2 * (self.elves.contains(&(x + 1, y - 1)) as u8))
            | (4 * (self.elves.contains(&(x + 1, y)) as u8))
            | (8 * (self.elves.contains(&(x + 1, y + 1)) as u8))
            | (16 * (self.elves.contains(&(x, y + 1)) as u8))
            | (32 * (self.elves.contains(&(x - 1, y + 1)) as u8))
            | (64 * (self.elves.contains(&(x - 1, y)) as u8))
            | (128 * (self.elves.contains(&(x - 1, y - 1)) as u8))
    }
}

#[derive(Default)]
enum Proposal {
    #[default]
    None,
    Elf(isize, isize),
    Multiple,
}

impl Proposal {
    fn insert(&mut self, x: isize, y: isize) {
        match self {
            Proposal::None => *self = Proposal::Elf(x, y),
            Proposal::Elf(_, _) => *self = Proposal::Multiple,
            Proposal::Multiple => (),
        }
    }
}
