use crate::common::*;
use std::collections::HashMap;

enum Operation<'a> {
    Number(isize),
    Op(&'a str, Op, &'a str),
}

impl<'a> Operation<'a> {
    fn evaluate(&self, monkeys: &HashMap<&str, Operation<'a>>) -> isize {
        match self {
            Operation::Number(n) => *n,
            Operation::Op(l, op, r) => op.evaluate(
                monkeys.get(l).unwrap().evaluate(monkeys),
                monkeys.get(r).unwrap().evaluate(monkeys),
            ),
        }
    }
}

impl<'a> From<&'a str> for Operation<'a> {
    fn from(s: &'a str) -> Self {
        let mut op = s.split(' ');
        let lhs = op.next().unwrap();
        match op.next() {
            None => Self::Number(lhs.parse().unwrap()),
            Some(operator) => Self::Op(lhs, operator.parse().unwrap(), op.next().unwrap()),
        }
    }
}

pub fn run(input: &str) -> isize {
    let monkeys: HashMap<_, Operation> = input
        .lines()
        .map(|line| {
            let (name, op) = line.split_once(": ").unwrap();

            (name, op.into())
        })
        .collect();

    let root = monkeys.get("root").unwrap();

    root.evaluate(&monkeys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

        assert_eq!(run(example), 152);
    }
}
