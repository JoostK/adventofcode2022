use shared::direction::{move_point, Direction};
use shared::iterator::ToArray;
use shared::point::Point;
use std::fmt::{Display, Formatter, Write};

pub struct Grid {
    pub zero: Point,
    pub width: usize,
    pub height: usize,
    pub particles: Vec<Particle>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut columns = 0;
        for p in self.particles.iter() {
            write!(f, "{}", p)?;

            columns += 1;
            if columns == self.width {
                columns = 0;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Grid {
    pub fn at(&self, pt: Point) -> &Particle {
        &self.particles[(pt.y - self.zero.y) as usize * self.width + (pt.x - self.zero.x) as usize]
    }

    pub fn at_mut(&mut self, pt: Point) -> &mut Particle {
        &mut self.particles
            [(pt.y - self.zero.y) as usize * self.width + (pt.x - self.zero.x) as usize]
    }

    pub fn neighbour(&self, pt: &Point, dir: Direction) -> Option<Point> {
        let p = move_point(pt, dir);
        let relative = p - self.zero;
        if relative.x < 0
            || relative.x == self.width as isize
            || relative.y < 0
            || relative.y == self.height as isize
        {
            None
        } else {
            Some(p)
        }
    }
}

#[derive(Copy, Clone)]
pub enum Particle {
    Air,
    Rock,
    Sand,
}

impl Display for Particle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Particle::Air => f.write_char('.'),
            Particle::Rock => f.write_char('#'),
            Particle::Sand => f.write_char('o'),
        }
    }
}

pub struct Rock {
    pub lines: Vec<(Point, Point)>,
}

pub fn parse_rocks(input: &str, sand: Point) -> (Point, Point, Vec<Rock>) {
    let mut rocks = Vec::new();
    let (mut min_x, mut min_y) = sand.into();
    let (mut max_x, mut max_y) = sand.into();

    for line in input.lines() {
        let (min, max, rock) = parse_rock(line);

        min_x = min_x.min(min.x);
        min_y = min_y.min(min.y);
        max_x = max_x.max(max.x);
        max_y = max_y.max(max.y);

        rocks.push(rock);
    }

    (Point::new(min_x, min_y), Point::new(max_x, max_y), rocks)
}

fn parse_rock(line: &str) -> (Point, Point, Rock) {
    let mut coords = line.split(" -> ").map(|p| {
        let [x, y] = p
            .split(',')
            .map(|d| d.parse::<isize>().unwrap())
            .collect_array();
        (x, y)
    });

    let mut lines = Vec::new();
    let mut prev @ (x, y) = coords.next().unwrap();
    let (mut min_x, mut min_y) = (x, y);
    let (mut max_x, mut max_y) = (x, y);
    for (x, y) in coords {
        lines.push((Point::new(prev.0, prev.1), Point::new(x, y)));
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        prev = (x, y);
    }

    (
        Point::new(min_x, min_y),
        Point::new(max_x, max_y),
        Rock { lines },
    )
}

pub fn simulate_sand(grid: &Grid, spawn: Point) -> Option<Point> {
    let mut p = spawn;
    loop {
        let down = grid.neighbour(&p, Direction::Down)?;
        if matches!(grid.at(down), Particle::Air) {
            p = down;
            continue;
        }

        let left = grid.neighbour(&down, Direction::Left)?;
        if matches!(grid.at(left), Particle::Air) {
            p = left;
            continue;
        }
        let right = grid.neighbour(&down, Direction::Right)?;
        if matches!(grid.at(right), Particle::Air) {
            p = right;
            continue;
        }

        return Some(p);
    }
}
