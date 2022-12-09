use crate::common::*;
use std::collections::BTreeSet;

const N: usize = 10;

pub fn run(input: &str) -> usize {
    let mut visited: BTreeSet<(i32, i32)> = Default::default();
    let mut rope = [(0i32, 0i32); N];

    visited.insert(rope[N - 1]);

    for line in input.lines() {
        let (dir, amount) = line.split_once(' ').unwrap();
        let (dir_x, dir_y) = decode_direction(dir);
        let amount: usize = amount.parse().unwrap();

        for _ in 0..amount {
            rope[0].0 += dir_x;
            rope[0].1 += dir_y;

            for i in 1..rope.len() {
                let head = rope[i - 1];
                let tail = &mut rope[i];
                let dx = head.0 - tail.0;
                let dy = head.1 - tail.1;
                if dx.abs() < 2 && dy.abs() < 2 {
                    break;
                }
                let mx = dx - dx.signum();
                let my = dy - dy.signum();
                *tail = (head.0 - mx, head.1 - my);
            }

            visited.insert(rope[N - 1]);
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
