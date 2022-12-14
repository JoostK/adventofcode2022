use crate::common::*;
use shared::point::Point;

pub fn run(input: &str) -> usize {
    let sand_start = Point::new(500, 0);
    let (mut min, mut max, rocks) = parse_rocks(input, sand_start);

    let floor = max.y + 2;
    max.y = floor - 1;

    // Ensure that at least `floor` amount of space is available on both sides of the starting point
    // to allow filling the whole triangle below it. Also allocate additional vertical buffers to
    // avoid checks for having reached the edges.
    min.x = min.x.min(sand_start.x - floor - 1);
    max.x = max.x.max(sand_start.x + floor + 1);

    let width = (max.x - min.x + 1) as usize;
    let height = (max.y - min.y + 1) as usize;
    let mut grid = Grid::new(min, width, height);

    place_rocks(&mut grid, rocks);

    let mut units = 1;

    let mut q = vec![grid.index(sand_start)];
    while let Some(p) = q.pop() {
        let down = p + grid.width;

        let fall = (down / grid.width) as isize != floor - 1;

        let (bucket, shift) = grid.bit_mut(down);
        if shift != 0 && shift != BITS - 1 {
            let s = shift - 1;
            // Fast path where all three target positions are stored in the same bucket; we can then
            // query their status at once.
            let t = (*bucket >> s) & 0b111;
            if t == 0b111 {
                continue;
            }

            *bucket |= 0b111 << s;

            if t & 0b010 == 0 {
                units += 1;

                if fall {
                    q.push(down);
                }
            }

            debug_assert_ne!((p % grid.width) as isize, grid.zero.x);
            if t & 0b001 == 0 {
                units += 1;

                if fall {
                    q.push(down - 1);
                }
            }

            debug_assert_ne!(
                (p % grid.width) as isize,
                (grid.width as isize - grid.zero.x)
            );
            if t & 0b100 == 0 {
                units += 1;

                if fall {
                    q.push(down + 1);
                }
            }
        } else {
            if grid.mark(down) {
                units += 1;

                if fall {
                    q.push(down);
                }
            }

            debug_assert_ne!((p % grid.width) as isize, grid.zero.x);
            let left = down - 1;
            if grid.mark(left) {
                units += 1;

                if fall {
                    q.push(left);
                }
            }

            debug_assert_ne!(
                (p % grid.width) as isize,
                (grid.width as isize - grid.zero.x)
            );
            let right = down + 1;
            if grid.mark(right) {
                units += 1;

                if fall {
                    q.push(right);
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
