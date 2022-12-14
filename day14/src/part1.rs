use crate::common::*;
use shared::point::Point;

pub fn run(input: &str) -> usize {
    let sand_start = Point::new(500, 0);
    let (min, max, rocks) = parse_rocks(input, sand_start);

    let width = (max.x - min.x + 1) as usize;
    let height = (max.y - min.y + 1) as usize;
    let mut grid = Grid::new(min, width, height);

    place_rocks(&mut grid, rocks);

    let mut units = 0;
    let mut start = sand_start;
    while let Some((p, resume)) = simulate_sand(&grid, start) {
        grid.set(p);
        start = resume.unwrap_or(sand_start);

        units += 1;
    }

    units
}

fn simulate_sand(grid: &Grid, spawn: Point) -> Option<(Point, Option<Point>)> {
    let mut p = spawn;
    let mut resume = None;
    loop {
        let y = p.y as usize + 1;
        if y == grid.height {
            return None;
        }

        let (bucket, shift) = grid.bit(p.x as usize, y);
        if shift != 0 && shift != BITS - 1 {
            // Fast path where all three target positions are stored in the same bucket; we can then
            // query their status at once.
            let t = bucket >> (shift - 1);

            if t & 0b010 == 0 {
                resume = Some(p);
                p = Point::new(p.x, y as isize);
                continue;
            }

            if p.x == grid.zero.x {
                return None;
            }
            if t & 0b001 == 0 {
                resume = Some(p);
                p = Point::new(p.x - 1, y as isize);
                continue;
            }

            if p.x == (grid.width as isize - grid.zero.x) {
                return None;
            }
            if t & 0b100 == 0 {
                resume = Some(p);
                p = Point::new(p.x + 1, y as isize);
                continue;
            }
        } else {
            let down = Point::new(p.x, y as isize);
            if !grid.is_set(down) {
                resume = Some(p);
                p = down;
                continue;
            }

            if p.x != grid.zero.x {
                let left = Point::new(down.x - 1, down.y);
                if !grid.is_set(left) {
                    resume = Some(p);
                    p = left;
                    continue;
                }
            }
            if p.x != (grid.width as isize - grid.zero.x) {
                let right = Point::new(down.x + 1, down.y);
                if !grid.is_set(right) {
                    resume = Some(p);
                    p = right;
                    continue;
                }
            }
        };

        return Some((p, resume));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!(run(example), 24);
    }
}
