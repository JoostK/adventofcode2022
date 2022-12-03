use crate::common::*;

pub fn run(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() >> 1);

            let left = parse_set(left);
            let right = parse_set(right);

            *left
                .intersection(&right)
                .next()
                .expect("a single item must be duplicate") as usize
        })
        .sum()
}
