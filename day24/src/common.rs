use shared::direction::Direction;
use std::fmt::{Display, Formatter, Write};

#[derive(Clone)]
pub struct Blizzard {
    pub direction: Direction,
}

#[derive(Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub blizzards: Vec<Vec<Blizzard>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            let mut c = y * self.width;
            for _ in 0..self.width {
                let blizzards = &self.blizzards[c];
                let m = match blizzards.len() {
                    0 => '.',
                    1 => match blizzards.first().unwrap().direction {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    },
                    n => (n as u8 + b'0') as char,
                };
                f.write_char(m)?;
                c += 1;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut width = 0usize;
        let mut height = 0usize;
        for line in input.lines() {
            width = line.len() - 2;
            height += 1;
        }

        let mut blizzards = vec![Vec::new(); width * height];

        for (y, line) in input.lines().skip(1).enumerate() {
            let mut c = (y + 1) * width;
            for b in line[1..line.len()].bytes() {
                match b {
                    b'.' => (),
                    b'v' => blizzards[c].push(Blizzard {
                        direction: Direction::Down,
                    }),
                    b'^' => blizzards[c].push(Blizzard {
                        direction: Direction::Up,
                    }),
                    b'<' => blizzards[c].push(Blizzard {
                        direction: Direction::Left,
                    }),
                    b'>' => blizzards[c].push(Blizzard {
                        direction: Direction::Right,
                    }),
                    b'#' => break,
                    _ => panic!("unknown symbol"),
                }
                c += 1;
            }
        }

        Self {
            width,
            height,
            blizzards,
        }
    }
}

impl Grid {
    fn simulate(&self, into: &mut Grid) {
        for b in &mut into.blizzards {
            b.clear();
        }
        for (c, blizzards) in self.blizzards.iter().enumerate() {
            for b in blizzards {
                let new_c = match b.direction {
                    Direction::Left => {
                        if c % self.width == 0 {
                            c + self.width - 1
                        } else {
                            c - 1
                        }
                    }
                    Direction::Right => {
                        if c % self.width == self.width - 1 {
                            c + 1 - self.width
                        } else {
                            c + 1
                        }
                    }
                    Direction::Up => {
                        if c / self.width == 1 {
                            c + (self.width * (self.height - 2)) - self.width
                        } else {
                            c - self.width
                        }
                    }
                    Direction::Down => {
                        if c / self.width == self.height - 2 {
                            c + self.width - (self.width * (self.height - 2))
                        } else {
                            c + self.width
                        }
                    }
                };
                into.blizzards[new_c].push(b.clone());
            }
        }
    }

    fn is_free(&self, c: usize) -> bool {
        self.blizzards[c].is_empty()
    }
}

pub fn walk(start: usize, goal: usize, mut grid: Grid) -> (usize, Grid) {
    let mut depth = 0;

    let width = grid.width;
    let height = grid.height;

    let mut marked = vec![false; grid.width * grid.height];
    let mut next_marked = marked.clone();

    let mut pending = Vec::with_capacity(marked.len());
    let mut next_pending = pending.clone();

    pending.push(start);

    let mut next_grid = grid.clone();
    while !marked[goal] {
        grid.simulate(&mut next_grid);

        for &c in &pending {
            marked[c] = false;
            let x = c % width;
            let y = c / width;

            if y != height - 1 && x > 0 && !next_marked[c - 1] && next_grid.is_free(c - 1) {
                next_marked[c - 1] = true;
                next_pending.push(c - 1);
            }
            if y != 0 && x < width - 1 && !next_marked[c + 1] && next_grid.is_free(c + 1) {
                next_marked[c + 1] = true;
                next_pending.push(c + 1);
            }
            if y > 1 && !next_marked[c - width] && next_grid.is_free(c - width) {
                next_marked[c - width] = true;
                next_pending.push(c - width);
            }
            if y < height - 2 && !next_marked[c + width] && next_grid.is_free(c + width) {
                next_marked[c + width] = true;
                next_pending.push(c + width);
            }

            if !next_marked[c] && next_grid.is_free(c) {
                next_marked[c] = true;
                next_pending.push(c);
            }
        }

        pending.clear();

        std::mem::swap(&mut grid, &mut next_grid);
        std::mem::swap(&mut marked, &mut next_marked);
        std::mem::swap(&mut pending, &mut next_pending);
        depth += 1;
    }

    (depth, grid)
}
