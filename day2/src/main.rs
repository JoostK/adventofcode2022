mod part1;
mod part2;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn shape_score(shape: Shape) -> usize {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn play_score(you: Shape, opponent: Shape) -> usize {
    if you == opponent {
        return 3;
    }

    match (you, opponent) {
        (Shape::Rock, Shape::Scissors)
        | (Shape::Paper, Shape::Rock)
        | (Shape::Scissors, Shape::Paper) => 6,
        _ => 0,
    }
}

fn round_score(you: Shape, opponent: Shape) -> usize {
    shape_score(you) + play_score(you, opponent)
}

pub trait Assignment {
    fn play(line: &str) -> usize;
}

fn run<A: Assignment>() -> usize {
    let input = include_str!("input.txt");
    input.lines().map(A::play).sum()
}

fn main() {
    let score = run::<part1::Assignment>();
    println!("part 1: {}", score);

    let score = run::<part2::Assignment>();
    println!("part 2: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let score = run::<part1::Assignment>();
        assert_eq!(score, 9651);
    }

    #[test]
    fn part2() {
        let score = run::<part2::Assignment>();
        assert_eq!(score, 10560);
    }
}
