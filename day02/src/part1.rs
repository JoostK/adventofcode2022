use crate::{round_score, Shape};

fn parse_shape(input: char) -> Shape {
    match input {
        'A' | 'X' => Shape::Rock,
        'B' | 'Y' => Shape::Paper,
        'C' | 'Z' => Shape::Scissors,
        _ => panic!("invalid shape"),
    }
}

pub struct Assignment;

impl super::Assignment for Assignment {
    fn play(line: &str) -> usize {
        let mut chars = line.chars();
        let opponent = parse_shape(chars.next().expect("expected shape"));
        let you = parse_shape(chars.nth(1).expect("expected shape"));

        round_score(you, opponent)
    }
}
