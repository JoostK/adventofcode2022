use shared::iterator::ToArray;

#[derive(Debug)]
pub enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

impl Operation {
    pub fn apply(&self, old: usize) -> usize {
        match self {
            Operation::Add(op) => old + op,
            Operation::Multiply(op) => old * op,
            Operation::Square => old * old,
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<usize>,
    pub operation: Operation,
    pub divider: usize,
    pub positive: usize,
    pub negative: usize,
}

impl Monkey {
    pub fn throws_to(&self, item: usize) -> usize {
        if item % self.divider == 0 {
            self.positive
        } else {
            self.negative
        }
    }
}

/// Monkey 0:
//   Starting items: 64, 89, 65, 95
//   Operation: new = old * 7
//   Test: divisible by 3
//     If true: throw to monkey 4
//     If false: throw to monkey 1
pub fn parse(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(parse_monkey).collect()
}

fn parse_monkey(input: &str) -> Monkey {
    let [_, items, operation, test, positive, negative] = input.lines().collect_array();

    Monkey {
        items: items
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|id| id.parse().unwrap())
            .collect(),
        operation: parse_operation(operation),
        divider: test.rsplit_once(' ').unwrap().1.parse().unwrap(),
        positive: positive.rsplit_once(' ').unwrap().1.parse().unwrap(),
        negative: negative.rsplit_once(' ').unwrap().1.parse().unwrap(),
    }
}

fn parse_operation(input: &str) -> Operation {
    let mut parts = input.rsplitn(3, ' ');
    let operand = parts.next().unwrap();

    if operand == "old" {
        return Operation::Square;
    }

    match parts.next().unwrap() {
        "+" => Operation::Add(operand.parse().unwrap()),
        "*" => Operation::Multiply(operand.parse().unwrap()),
        op => panic!("invalid op {op}"),
    }
}

pub fn compute_monkey_business(mut inspections: Vec<usize>) -> usize {
    inspections.sort_by(|a, b| b.cmp(a));
    inspections.iter().take(2).product()
}
