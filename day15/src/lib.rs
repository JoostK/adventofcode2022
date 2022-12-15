mod common;
pub mod part1;
pub mod part2;

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let answer = part1::run(INPUT, 2_000_000);
        assert_eq!(answer, 4_665_948);
    }

    #[test]
    fn part2() {
        let answer = part2::run(INPUT, 4_000_000);
        assert_eq!(answer, 13_543_690_671_045);
    }
}
