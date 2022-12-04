use crate::common::*;

pub fn run(input: &str) -> usize {
    input
        .lines()
        .map(parse_sections)
        .filter(|[fst, snd]| {
            (fst.start >= snd.start && fst.end <= snd.end)
                || (snd.start >= fst.start && snd.end <= fst.end)
        })
        .count()
}
