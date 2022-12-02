use crate::{round_score, Shape};

#[derive(Eq, PartialEq, Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

fn parse_shape(input: char) -> Shape {
    match input {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        _ => panic!("invalid shape"),
    }
}

fn parse_outcome(input: char) -> Outcome {
    match input {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("invalid outcome"),
    }
}

fn determine_shape(opponent: Shape, outcome: Outcome) -> Shape {
    if outcome == Outcome::Draw {
        return opponent;
    }

    match (opponent, outcome) {
        (Shape::Rock, Outcome::Win) | (Shape::Scissors, Outcome::Lose) => Shape::Paper,
        (Shape::Paper, Outcome::Win) | (Shape::Rock, Outcome::Lose) => Shape::Scissors,
        (Shape::Scissors, Outcome::Win) | (Shape::Paper, Outcome::Lose) => Shape::Rock,
        _ => unreachable!("draw handled separately"),
    }
}

pub struct Assignment;

impl super::Assignment for Assignment {
    fn play(line: &str) -> usize {
        let mut chars = line.chars();
        let opponent = parse_shape(chars.next().expect("expected shape"));
        let outcome = parse_outcome(chars.nth(1).expect("expected outcome"));

        let you = determine_shape(opponent, outcome);

        round_score(you, opponent)
    }
}
