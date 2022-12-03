use crate::common::*;
use std::collections::HashSet;

pub fn run(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|lines| {
            lines
                .iter()
                .cloned()
                .map(parse_set)
                .reduce(|set, elf| HashSet::from_iter(set.intersection(&elf).cloned()))
                .and_then(|set| set.into_iter().next())
                .expect("a single item must be duplicate") as usize
        })
        .sum()
}
