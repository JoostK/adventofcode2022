use crate::common::*;
use crate::rock::{Rock, ROCKS};
use std::str::Bytes;

const TOTAL_DROPS: usize = 1_000_000_000_000;

struct Simulation {
    cave: Cave,
    dropped: usize,
    seen: Vec<Option<(Rock, usize, usize)>>,
}

impl Simulation {
    fn run(&mut self, jetstream: Bytes) -> usize {
        let mut rocks = ROCKS.iter().cycle();
        let mut jetstream = jetstream.enumerate().cycle().peekable();
        loop {
            let rock = *rocks.next().unwrap();
            let jet_index = jetstream.peek().unwrap().0;

            if let Some((remaining_cyclic_depth, pending_drops)) =
                self.detect_cycle(rock, jet_index)
            {
                let mut remaining_rock = rock;
                for _ in 0..pending_drops {
                    self.cave.drop_rock(remaining_rock, &mut jetstream);
                    remaining_rock = *rocks.next().unwrap();
                }

                return remaining_cyclic_depth + self.cave.depth();
            }

            self.seen[jet_index] = Some((rock, self.dropped, self.cave.depth()));

            self.cave.drop_rock(rock, &mut jetstream);
            self.dropped += 1;
        }
    }

    fn detect_cycle(&mut self, rock: Rock, jet_index: usize) -> Option<(usize, usize)> {
        let (r, cycle_start, prev_depth) = self.seen[jet_index]?;
        if r != rock {
            return None;
        }

        let now_depth = self.cave.depth();
        let cycle_depth = now_depth - prev_depth;
        let cycle_length = self.dropped - cycle_start;

        if cycle_depth > prev_depth
            || self.cave.lines[(now_depth - cycle_depth + 1)..=now_depth]
                != self.cave.lines[(prev_depth - cycle_depth + 1)..=prev_depth]
        {
            return None;
        }

        let remaining_drops = TOTAL_DROPS - self.dropped;
        let remaining_cyclic_depth = remaining_drops / cycle_length * cycle_depth;
        let pending_drops = remaining_drops % cycle_length;
        Some((remaining_cyclic_depth, pending_drops))
    }
}

pub fn run(input: &str) -> usize {
    let jetstream = input.lines().next().unwrap();

    let mut simulation = Simulation {
        cave: Cave::default(),
        dropped: 0,
        seen: vec![None; jetstream.len()],
    };

    let jetstream = jetstream.bytes();
    simulation.run(jetstream)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert_eq!(run(example), 1_514_285_714_288);
    }
}
