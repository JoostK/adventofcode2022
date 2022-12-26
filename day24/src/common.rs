use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Write};

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub left: VecDeque<Vec<bool>>,
    pub right: VecDeque<Vec<bool>>,
    pub up: VecDeque<Vec<bool>>,
    pub down: VecDeque<Vec<bool>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..(self.height - 2) {
            for x in 0..self.width {
                let left = self.left[x][y];
                let right = self.right[x][y];
                let up = self.up[y][x];
                let down = self.down[y][x];
                let m = match left as u8 + right as u8 + up as u8 + down as u8 {
                    0 => '.',
                    1 if left => '<',
                    1 if right => '>',
                    1 if up => '^',
                    1 => 'v',
                    n => (n as u8 + b'0') as char,
                };
                f.write_char(m)?;
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

        let mut left = VecDeque::with_capacity(width);
        let mut right = VecDeque::with_capacity(width);
        for _ in 0..width {
            left.push_back(vec![false; height - 2]);
            right.push_back(vec![false; height - 2]);
        }

        let mut up = VecDeque::with_capacity(height - 2);
        let mut down = VecDeque::with_capacity(height - 2);
        for _ in 0..(height - 2) {
            up.push_back(vec![false; width]);
            down.push_back(vec![false; width]);
        }

        for (y, line) in input.lines().skip(1).enumerate() {
            for (x, b) in line[1..line.len()].bytes().enumerate() {
                match b {
                    b'.' => (),
                    b'<' => left[x][y] = true,
                    b'>' => right[x][y] = true,
                    b'v' => down[y][x] = true,
                    b'^' => up[y][x] = true,
                    b'#' => break,
                    _ => panic!("unknown symbol"),
                }
            }
        }

        Self {
            width,
            height,
            left,
            right,
            up,
            down,
        }
    }
}

impl Grid {
    fn simulate(&mut self) {
        self.left.rotate_left(1);
        self.right.rotate_right(1);
        self.up.rotate_left(1);
        self.down.rotate_right(1);
    }

    fn is_free(&self, c: usize) -> bool {
        let x = c % self.width;
        let y = c / self.width;
        if y == 0 {
            return x == 0;
        } else if y == self.height - 1 {
            return x == self.width - 1;
        }
        let y = y - 1;
        !self.left[x][y] && !self.right[x][y] && !self.up[y][x] && !self.down[y][x]
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

    while !marked[goal] {
        grid.simulate();

        for &c in &pending {
            marked[c] = false;
            let x = c % width;
            let y = c / width;

            if y != height - 1 && x > 0 && !next_marked[c - 1] && grid.is_free(c - 1) {
                next_marked[c - 1] = true;
                next_pending.push(c - 1);
            }
            if y != 0 && x < width - 1 && !next_marked[c + 1] && grid.is_free(c + 1) {
                next_marked[c + 1] = true;
                next_pending.push(c + 1);
            }
            if y > 1 && !next_marked[c - width] && grid.is_free(c - width) {
                next_marked[c - width] = true;
                next_pending.push(c - width);
            }
            if y < height - 2 && !next_marked[c + width] && grid.is_free(c + width) {
                next_marked[c + width] = true;
                next_pending.push(c + width);
            }

            if !next_marked[c] && grid.is_free(c) {
                next_marked[c] = true;
                next_pending.push(c);
            }
        }

        pending.clear();

        std::mem::swap(&mut marked, &mut next_marked);
        std::mem::swap(&mut pending, &mut next_pending);
        depth += 1;
    }

    (depth, grid)
}
