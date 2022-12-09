use crate::common::*;
use std::collections::BTreeSet;

pub fn run(input: &str) -> usize {
    let mut visited: BTreeSet<(i32, i32)> = Default::default();
    let mut head = (0, 0);
    let mut tail = (0, 0);

    visited.insert(tail);

    for line in input.lines() {
        let (dir, amount) = line.split_once(' ').unwrap();
        let (dir_x, dir_y) = decode_direction(dir);
        let amount: usize = amount.parse().unwrap();

        for _ in 0..amount {
            head.0 += dir_x;
            head.1 += dir_y;

            let dx = (head.0 - tail.0).abs();
            let dy = (head.1 - tail.1).abs();
            if dx < 2 && dy < 2 {
                continue;
            }
            if dx < dy {
                tail.0 = head.0;
                tail.1 = head.1 - dir_y;
            } else {
                tail.0 = head.0 - dir_x;
                tail.1 = head.1;
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
