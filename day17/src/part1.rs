use crate::common::*;
use crate::rock::ROCKS;

pub fn run(input: &str) -> usize {
    let mut cave = Cave::default();
    let mut rocks = ROCKS.iter().cycle();
    let mut jetstream = input.lines().next().unwrap().bytes().enumerate().cycle();

    for _ in 0..2022 {
        let rock = *rocks.next().unwrap();
        cave.drop_rock(rock, &mut jetstream);
    }

    cave.depth()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        assert_eq!(run(example), 3068);
    }
}
