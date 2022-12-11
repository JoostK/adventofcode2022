use crate::common::*;

pub fn run(input: &str) -> usize {
    let monkeys = parse(input);

    let mut items: Vec<_> = monkeys.iter().map(|m| m.items.clone()).collect();
    let mut inspections: Vec<_> = monkeys.iter().map(|_| 0usize).collect();

    let modulus: usize = monkeys.iter().map(|m| m.divider).product();

    for _ in 0..10_000 {
        for (id, monkey) in monkeys.iter().enumerate() {
            let taken = std::mem::take(&mut items[id]);
            inspections[id] += taken.len();
            for item in taken {
                let worry = monkey.operation.apply(item) % modulus;

                let next = monkey.throws_to(worry);
                items[next].push(worry);
            }
        }
    }

    compute_monkey_business(inspections)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        assert_eq!(run(example), 2_713_310_158);
    }
}
