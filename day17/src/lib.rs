mod common;
pub mod part1;
pub mod part2;
mod rock;

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let answer = part1::run(INPUT);
        assert_eq!(answer, 3059);
    }

    #[test]
    fn part2() {
        let answer = part2::run(INPUT);
        assert_eq!(answer, 1_500_874_635_587);
    }
}
