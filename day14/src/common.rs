use shared::iterator::ToArray;
use shared::point::Point;

pub const BITS: usize = 64;
pub const SHIFT: u32 = BITS.trailing_zeros();
pub const MASK: usize = (1 << SHIFT) - 1;

pub struct Grid {
    pub particles: Vec<u64>,
    pub zero: Point,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(zero: Point, width: usize, height: usize) -> Self {
        Self {
            zero,
            width,
            height,
            particles: vec![0; ((width * height) >> SHIFT) + 1],
        }
    }

    #[inline]
    pub fn is_set(&self, pt: Point) -> bool {
        let index = self.index(pt);
        let bucket = index >> SHIFT;
        let bit = 1 << (index & MASK);
        self.particles[bucket] & bit != 0
    }

    #[inline(never)]
    pub fn set(&mut self, pt: Point) {
        let index = self.index(pt);
        let bucket = index >> SHIFT;
        let bit = 1 << (index & MASK);
        self.particles[bucket] |= bit;
    }

    #[inline]
    pub fn mark(&mut self, pt: Point) -> bool {
        let index = self.index(pt);
        let bucket = index >> SHIFT;
        let bit = 1 << (index & MASK);
        let set = self.particles[bucket] & bit == 0;
        self.particles[bucket] |= bit;
        set
    }

    pub fn bit(&self, x: usize, y: usize) -> (u64, usize) {
        let index = y * self.width + (x - self.zero.x as usize);
        let bucket = index >> SHIFT;
        let shift = index & MASK;
        (self.particles[bucket], shift)
    }

    pub fn bit_mut(&mut self, x: usize, y: usize) -> (&mut u64, usize) {
        let index = y * self.width + (x - self.zero.x as usize);
        let bucket = index >> SHIFT;
        let shift = index & MASK;
        (&mut self.particles[bucket], shift)
    }

    fn index(&self, pt: Point) -> usize {
        (pt.y - self.zero.y) as usize * self.width + (pt.x - self.zero.x) as usize
    }
}

pub type Rock = Vec<(Point, Point)>;

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
        p.split(',')
            .map(|d| d.parse::<isize>().unwrap())
            .collect_array()
    });

    let mut lines = Vec::new();
    let mut prev @ [x, y] = coords.next().unwrap();
    let (mut min_x, mut min_y) = (x, y);
    let (mut max_x, mut max_y) = (x, y);
    for [x, y] in coords {
        lines.push((Point::new(prev[0], prev[1]), Point::new(x, y)));
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        prev = [x, y];
    }

    (Point::new(min_x, min_y), Point::new(max_x, max_y), lines)
}

#[inline]
pub fn place_rocks(grid: &mut Grid, rocks: Vec<Rock>) {
    for rock in rocks {
        for (s, e) in rock {
            let dir = (e - s).map(isize::signum);
            let mut c = s;
            loop {
                grid.set(c);
                if c == e {
                    break;
                }
                c += dir;
            }
        }
    }
}
