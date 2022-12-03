use crate::common::*;

pub fn run(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() >> 1);

            Bag::new(left).intersect(Bag::new(right)).priority() as usize
        })
        .sum()
}
