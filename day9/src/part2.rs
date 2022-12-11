use crate::common::*;
use shared::point::Point;
use std::collections::BTreeSet;

const N: usize = 10;

pub fn run(input: &str) -> usize {
    let mut visited: BTreeSet<Point> = Default::default();
    let mut knots = [Point::default(); N];

    visited.insert(knots[N - 1]);

    for line in input.lines() {
        let (dir, amount) = line.split_once(' ').unwrap();
        let dir: Point = decode_direction(dir).into();
        let amount: usize = amount.parse().unwrap();

        for _ in 0..amount {
            knots[0] += dir;

            let mut reached_tail = true;
            for i in 1..N {
                let head = knots[i - 1];
                let tail = &mut knots[i];
                let delta = head - *tail;
                if delta.x.abs() < 2 && delta.y.abs() < 2 {
                    reached_tail = false;
                    break;
                }
                *tail = head - (delta - delta.map(isize::signum));
            }

            if reached_tail {
                visited.insert(knots[N - 1]);
            }
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let example = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(run(example), 1);
    }

    #[test]
    fn example2() {
        let example = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        assert_eq!(run(example), 36);
    }
}
