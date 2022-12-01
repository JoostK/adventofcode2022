mod part1;
mod part2;

pub trait Assignment: Default {
    fn finish(self) -> Self;
    fn add(self, calories: usize) -> Self;
    fn total_calories(&self) -> usize;
}

fn run<A: Assignment>() -> usize {
    let input = include_str!("input.txt");
    input
        .lines()
        .fold(A::default(), |acc, line| {
            if line.is_empty() {
                acc.finish()
            } else {
                acc.add(line.parse().expect("line should contain an integer"))
            }
        })
        .total_calories()
}

fn main() {
    let max_calories = run::<part1::Accumulator>();
    println!("part 1: {}", max_calories);

    let max_calories = run::<part2::Accumulator>();
    println!("part 2: {}", max_calories);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let max_calories = run::<part1::Accumulator>();
        assert_eq!(max_calories, 69206);
    }

    #[test]
    fn part2() {
        let max_calories = run::<part2::Accumulator>();
        assert_eq!(max_calories, 197400);
    }
}
