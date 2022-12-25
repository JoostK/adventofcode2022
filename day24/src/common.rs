use shared::direction::Direction;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};

#[derive(Clone)]
pub struct Blizzard {
    pub direction: Direction,
}

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
    fn simulate(&self) -> Self {
        let mut clone = Grid {
            width: self.width,
            height: self.height,
            blizzards: vec![Vec::new(); self.blizzards.len()],
        };
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
                clone.blizzards[new_c].push(b.clone());
            }
        }

        clone
    }

    fn is_free(&self, c: usize) -> bool {
        self.blizzards[c].is_empty()
    }
}

pub fn walk(yous: HashSet<usize>, goal: usize, begin: Grid) -> (usize, Grid) {
    if yous.contains(&goal) {
        return (0, begin);
    }
    let next = begin.simulate();

    let width = begin.width;
    let height = begin.height;

    let mut next_yous = HashSet::new();
    for you in yous {
        let x = you % width;
        let y = you / width;

        if y != 0 && x < width - 1 && next.is_free(you + 1) {
            next_yous.insert(you + 1);
        }

        if y < height - 2 && next.is_free(you + width) {
            next_yous.insert(you + width);
        }

        if y != height - 1 && x > 0 && next.is_free(you - 1) {
            next_yous.insert(you - 1);
        }

        if y > 1 && next.is_free(you - width) {
            next_yous.insert(you - width);
        }

        if next.is_free(you) {
            next_yous.insert(you);
        }
    }

    let (distance, grid) = walk(next_yous, goal, next);
    (distance + 1, grid)
}
