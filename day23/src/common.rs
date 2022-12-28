use std::str::FromStr;

const LEN: usize = 1 << 16;
const OFFSET: i16 = 1 << 7;
const X: u16 = 1 << 0;
const Y: u16 = 1 << 8;

pub struct Grid {
    elves: Vec<bool>,
    proposals: Vec<Proposal>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut elves = vec![false; LEN];
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.bytes().enumerate() {
                if c == b'#' {
                    elves[Position::new(x as i8, y as i8).index()] = true;
                }
            }
        }

        Ok(Self {
            elves,
            proposals: vec![Proposal::None; LEN],
        })
    }
}

impl Grid {
    pub fn simulate(&mut self, round: usize) -> bool {
        let first = round % 4;
        for (pos, _) in self.elves.iter().enumerate().filter(|(_, b)| **b) {
            let pos = Position::from_index(pos);

            let north = pos.north();
            let east = pos.east();
            let south = pos.south();
            let west = pos.west();

            let n = self.elves[north.index()];
            let ne = self.elves[north.east().index()];
            let e = self.elves[east.index()];
            let se = self.elves[south.east().index()];
            let s = self.elves[south.index()];
            let sw = self.elves[south.west().index()];
            let w = self.elves[west.index()];
            let nw = self.elves[north.west().index()];

            match (
                first,
                nw || n || ne,
                se || s || sw,
                sw || w || nw,
                ne || e || se,
            ) {
                (_, false, false, false, false) => (),
                //
                (0, false, _, _, _) => self.proposals[north.index()].insert(pos),
                (0, _, false, _, _) => self.proposals[south.index()].insert(pos),
                (0, _, _, false, _) => self.proposals[west.index()].insert(pos),
                (0, _, _, _, false) => self.proposals[east.index()].insert(pos),
                //
                (1, _, false, _, _) => self.proposals[south.index()].insert(pos),
                (1, _, _, false, _) => self.proposals[west.index()].insert(pos),
                (1, _, _, _, false) => self.proposals[east.index()].insert(pos),
                (1, false, _, _, _) => self.proposals[north.index()].insert(pos),
                //
                (2, _, _, false, _) => self.proposals[west.index()].insert(pos),
                (2, _, _, _, false) => self.proposals[east.index()].insert(pos),
                (2, false, _, _, _) => self.proposals[north.index()].insert(pos),
                (2, _, false, _, _) => self.proposals[south.index()].insert(pos),
                //
                (3, _, _, _, false) => self.proposals[east.index()].insert(pos),
                (3, false, _, _, _) => self.proposals[north.index()].insert(pos),
                (3, _, false, _, _) => self.proposals[south.index()].insert(pos),
                (3, _, _, false, _) => self.proposals[west.index()].insert(pos),
                //
                _ => (),
            }
        }

        let mut moved = false;
        for (pos, proposal) in self.proposals.iter_mut().enumerate() {
            match proposal {
                Proposal::None => (),
                Proposal::Elf(elf) => {
                    self.elves[pos] = true;
                    self.elves[elf.index()] = false;
                    moved = true;
                    *proposal = Proposal::None;
                }
                Proposal::Multiple => *proposal = Proposal::None,
            }
        }

        !moved
    }

    pub fn elves(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        self.elves
            .iter()
            .enumerate()
            .filter(|(_, e)| **e)
            .map(|(p, _)| Position::from_index(p).into())
    }
}

#[derive(Copy, Clone)]
struct Position(u16);

impl Position {
    fn new(x: i8, y: i8) -> Self {
        Self(((x as i16 + OFFSET) | (y as i16 + OFFSET) << 8) as u16)
    }

    fn from_index(i: usize) -> Self {
        Self(i as u16)
    }

    fn north(&self) -> Self {
        Self(self.0 - Y)
    }

    fn east(&self) -> Self {
        Self(self.0 + X)
    }

    fn south(&self) -> Self {
        Self(self.0 + Y)
    }

    fn west(&self) -> Self {
        Self(self.0 - X)
    }

    fn index(&self) -> usize {
        self.0 as usize
    }
}

impl From<Position> for (isize, isize) {
    fn from(p: Position) -> Self {
        (
            unsafe { std::mem::transmute::<u16, i16>((p.0 & 0xff) as u16) - OFFSET } as isize,
            unsafe { std::mem::transmute::<u16, i16>((p.0 >> 8) as u16) - OFFSET } as isize,
        )
    }
}

#[derive(Copy, Clone)]
enum Proposal {
    None,
    Elf(Position),
    Multiple,
}

impl Proposal {
    fn insert(&mut self, p: Position) {
        match self {
            Proposal::None => *self = Proposal::Elf(p),
            Proposal::Elf(_) => *self = Proposal::Multiple,
            Proposal::Multiple => (),
        }
    }
}
