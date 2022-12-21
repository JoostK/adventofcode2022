use crate::common::Op;
use std::collections::HashMap;

enum Operation<'a> {
    Number(isize),
    Op(&'a str, Op, &'a str),
    Human,
}

impl Op {
    fn solve_rhs(&self, lhs: isize, outcome: isize) -> isize {
        match *self {
            Op::Add => outcome - lhs,
            Op::Sub => lhs - outcome,
            Op::Mul => outcome / lhs,
            Op::Div => lhs / outcome,
        }
    }

    fn solve_lhs(&self, rhs: isize, outcome: isize) -> isize {
        match *self {
            Op::Add => outcome - rhs,
            Op::Sub => outcome + rhs,
            Op::Mul => outcome / rhs,
            Op::Div => outcome * rhs,
        }
    }
}

trait Solver {
    fn solve(&self, outcome: isize) -> isize;

    fn solve_equality(&self) -> isize;
}

struct Human;

impl Solver for Human {
    fn solve(&self, outcome: isize) -> isize {
        outcome
    }

    fn solve_equality(&self) -> isize {
        unimplemented!("a human does not represent an equality");
    }
}

enum KnownOperand {
    Lhs(isize),
    Rhs(isize),
}

struct Partial {
    known: KnownOperand,
    op: Op,
    solver: Box<dyn Solver>,
}

impl Partial {
    fn new(known: KnownOperand, op: Op, solver: Box<dyn Solver>) -> Self {
        Self { known, op, solver }
    }
}

impl Solver for Partial {
    fn solve(&self, outcome: isize) -> isize {
        let solution = match self.known {
            KnownOperand::Lhs(v) => self.op.solve_rhs(v, outcome),
            KnownOperand::Rhs(v) => self.op.solve_lhs(v, outcome),
        };

        #[cfg(debug_assertions)]
        match self.known {
            KnownOperand::Lhs(v) => assert_eq!(self.op.evaluate(v, solution), outcome),
            KnownOperand::Rhs(v) => assert_eq!(self.op.evaluate(solution, v), outcome),
        };

        self.solver.solve(solution)
    }

    fn solve_equality(&self) -> isize {
        match self.known {
            KnownOperand::Lhs(v) => self.solver.solve(v),
            KnownOperand::Rhs(v) => self.solver.solve(v),
        }
    }
}

impl<'a> Operation<'a> {
    fn evaluate(&self, monkeys: &HashMap<&str, Operation<'a>>) -> Result<isize, Box<dyn Solver>> {
        match self {
            Operation::Number(n) => Ok(*n),
            Operation::Op(l, op, r) => {
                let lhs = monkeys.get(l).unwrap().evaluate(monkeys);
                let rhs = monkeys.get(r).unwrap().evaluate(monkeys);

                match (lhs, rhs) {
                    (Ok(l), Ok(r)) => Ok(op.evaluate(l, r)),
                    (Ok(l), Err(r)) => Err(Box::new(Partial::new(KnownOperand::Lhs(l), *op, r))),
                    (Err(l), Ok(r)) => Err(Box::new(Partial::new(KnownOperand::Rhs(r), *op, l))),
                    _ => panic!("both unknown"),
                }
            }
            Operation::Human => Err(Box::new(Human)),
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
    let monkeys: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (name, op) = line.split_once(": ").unwrap();
            if name == "humn" {
                return (name, Operation::Human);
            }

            (name, op.into())
        })
        .collect();

    let root = monkeys.get("root").unwrap();
    let q = root.evaluate(&monkeys).unwrap_err();
    q.solve_equality()
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

        assert_eq!(run(example), 301);
    }
}
