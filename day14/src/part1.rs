use crate::common::*;
use shared::point::Point;

pub fn run(input: &str) -> usize {
    let sand_start = Point::new(500, 0);
    let (mut min, mut max, rocks) = parse_rocks(input, sand_start);

    // Allocate vertical buffers to avoid checks for having reached the edges.
    min.x -= 1;
    max.x += 1;

    let width = (max.x - min.x + 1) as usize;
    let height = (max.y - min.y + 1) as usize;
    let mut grid = Grid::new(min, width, height);

    place_rocks(&mut grid, rocks);

    let mut units = 0;
    let mut path = vec![sand_start];
    while let Some(p) = simulate_sand(&grid, &mut path) {
        grid.set(p);

        units += 1;
    }

    units
}

fn simulate_sand(grid: &Grid, path: &mut Vec<Point>) -> Option<Point> {
    loop {
        let p = *path.last()?;
        let y = p.y as usize + 1;
        if y == grid.height {
            return None;
        }

        let (bucket, shift) = grid.bit(p.x as usize, y);
        if shift != 0 && shift != BITS - 1 {
            // Fast path where all three target positions are stored in the same bucket; we can then
            // query their status at once.
            // Note: this hardly helps for part one. In fact, storing just a Vec<bool> without any
            // bit fiddling is slightly faster even but this would require a different Grid
            // implementation compared to part two, where this optimization does show a ~13%
            // performance improvement.
            let t = bucket >> (shift - 1);

            if t & 0b010 == 0 {
                path.push(Point::new(p.x, y as isize));
                continue;
            }

            debug_assert_ne!(p.x, grid.zero.x);
            if t & 0b001 == 0 {
                path.push(Point::new(p.x - 1, y as isize));
                continue;
            }

            debug_assert_ne!(p.x, (grid.width as isize - grid.zero.x));
            if t & 0b100 == 0 {
                path.push(Point::new(p.x + 1, y as isize));
                continue;
            }
        } else {
            let down = Point::new(p.x, y as isize);
            if !grid.is_set(down) {
                path.push(down);
                continue;
            }

            debug_assert_ne!(p.x, grid.zero.x);
            let left = Point::new(down.x - 1, down.y);
            if !grid.is_set(left) {
                path.push(left);
                continue;
            }

            debug_assert_ne!(p.x, (grid.width as isize - grid.zero.x));
            let right = Point::new(down.x + 1, down.y);
            if !grid.is_set(right) {
                path.push(right);
                continue;
            }
        };

        path.pop();

        return Some(p);
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
