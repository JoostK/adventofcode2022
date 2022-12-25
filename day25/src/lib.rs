extern crate core;

mod common;
pub mod part1;
pub mod part2;

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let answer = part1::run(INPUT);
        assert_eq!(answer, "2-=12=2-2-2-=0012==2");
    }

    #[test]
    fn part2() {
        let answer = part2::run(INPUT);
        assert_eq!(answer, 0);
    }
}
