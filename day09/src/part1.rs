use crate::common::*;
use shared::point::Point;
use std::collections::BTreeSet;

pub fn run(input: &str) -> usize {
    let mut visited: BTreeSet<Point> = Default::default();
    let mut head = Point::default();
    let mut tail = Point::default();

    visited.insert(tail);

    for line in input.lines() {
        let (dir, amount) = line.split_once(' ').unwrap();
        let dir: Point = decode_direction(dir).into();
        let amount: usize = amount.parse().unwrap();

        for _ in 0..amount {
            head += dir;

            let delta = (head - tail).map(isize::abs);
            if delta.x < 2 && delta.y < 2 {
                continue;
            }
            if delta.x < delta.y {
                tail = (head.x, head.y - dir.y).into();
            } else {
                tail = (head.x - dir.x, head.y).into();
            }
            visited.insert(tail);
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(run(example), 13);
    }
}
