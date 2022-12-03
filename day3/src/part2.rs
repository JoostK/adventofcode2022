use crate::common::*;

pub fn run(input: &str) -> usize {
    input
        .lines()
        .map(Bag::new)
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|lines| {
            lines
                .iter()
                .cloned()
                .reduce(Bag::intersect)
                .expect("expected at least one bag")
                .priority() as usize
        })
        .sum()
}
