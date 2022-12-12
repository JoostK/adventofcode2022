use shared::point::Point;
use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use std::mem::transmute;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u16)]
pub enum Direction {
    Up = 0xff00,
    Down = 0x0100,
    Left = 0x00ff,
    Right = 0x0001,
}

const HEIGHT_MASK: u8 = (1 << 5) - 1;
const VISITED_MASK: u8 = 1 << 5;

pub struct Position(Cell<u8>);

impl Position {
    pub fn new(height: u8) -> Self {
        Self(Cell::new(height))
    }

    pub fn height(&self) -> i8 {
        (self.0.get() & HEIGHT_MASK) as i8
    }

    pub fn mark_visited(&self) -> bool {
        let bits = self.0.get();
        let visited = bits & VISITED_MASK != 0;
        self.0.set(bits | VISITED_MASK);
        visited
    }

    pub fn visited(&self) -> bool {
        self.0.get() & VISITED_MASK != 0
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Position")
            .field("height", &self.height())
            .field("visited", &self.visited())
            .finish()
    }
}

pub fn move_point(pt: &Point, dir: Direction) -> Point {
    let d = unsafe { transmute::<Direction, u16>(dir) };
    let dx = d as i8 as isize;
    let dy = (d >> 8) as i8 as isize;
    Point {
        x: pt.x + dx,
        y: pt.y + dy,
    }
}

#[derive(Default)]
pub struct Grid {
    width: usize,
    height: usize,
    buffer: Vec<Position>,
}

impl Grid {
    pub fn at(&self, pt: Point) -> &Position {
        &self.buffer[pt.y as usize * self.width + pt.x as usize]
    }

    pub fn neighbour(&self, pt: &Point, dir: Direction) -> Option<Point> {
        let p = move_point(pt, dir);
        if p.x < 0 || p.x == self.width as isize || p.y < 0 || p.y == self.height as isize {
            None
        } else {
            Some(p)
        }
    }
}

pub fn parse(input: &str) -> (Grid, Point, Point) {
    let mut grid = Grid::default();
    let mut start = Point::default();
    let mut end = Point::default();
    for line in input.lines() {
        for (index, &b) in line.as_bytes().iter().enumerate() {
            let height = if b == b'S' {
                start = Point::new(index as isize, grid.height as isize);
                b'a'
            } else if b == b'E' {
                end = Point::new(index as isize, grid.height as isize);
                b'z'
            } else {
                b
            } - b'a';
            grid.buffer.push(Position::new(height));
        }
        grid.height += 1;
        grid.width = line.len();
    }

    (grid, start, end)
}