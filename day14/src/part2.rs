use crate::common::*;
use shared::point::Point;

pub fn run(input: &str) -> usize {
    let sand_start = Point::new(500, 0);
    let (mut min, mut max, rocks) = parse_rocks(input, sand_start);

    let floor = max.y + 2;
    max.y = floor - 1;

    min.x = min.x.min(sand_start.x - floor);
    max.x = max.x.max(sand_start.x + floor);

    let width = (max.x - min.x + 1) as usize;
    let height = (max.y - min.y + 1) as usize;
    let mut grid = Grid::new(min, width, height);

    place_rocks(&mut grid, rocks);

    let mut units = 1;

    let mut q = vec![sand_start];
    while let Some(p) = q.pop() {
        let down = Point::new(p.x, p.y + 1);

        let fall = down.y != floor - 1;

        let (bucket, shift) = grid.bit_mut(p.x as usize, down.y as usize);
        if shift != 0 && shift != BITS - 1 {
            let s = shift - 1;
            // Fast path where all three target positions are stored in the same bucket; we can then
            // query their status at once.
            let t = *bucket >> s;

            *bucket |= 0b111 << s;

            if t & 0b010 == 0 {
                units += 1;

                if fall {
                    q.push(down);
                }
            }

            if p.x != grid.zero.x && t & 0b001 == 0 {
                units += 1;

                if fall {
                    q.push(Point::new(down.x - 1, down.y));
                }
            }

            if p.x != (grid.width as isize - grid.zero.x) && t & 0b100 == 0 {
                units += 1;

                if fall {
                    q.push(Point::new(down.x + 1, down.y));
                }
            }
        } else {
            if grid.mark(down) {
                units += 1;

                if fall {
                    q.push(down);
                }
            }

            if p.x != grid.zero.x {
                let left = Point::new(down.x - 1, down.y);
                if grid.mark(left) {
                    units += 1;

                    if fall {
                        q.push(left);
                    }
                }
            }
            if p.x != (grid.width as isize - grid.zero.x) {
                let right = Point::new(down.x + 1, down.y);
                if grid.mark(right) {
                    units += 1;

                    if fall {
                        q.push(right);
                    }
                }
            }
        }
    }

    units
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!(run(example), 93);
    }
}
