use crate::common::*;

pub fn run(input: &str) -> usize {
    input
        .lines()
        .map(parse_sections)
        .filter(|[fst, snd]| snd.start <= fst.end && snd.end >= fst.start)
        .count()
}
