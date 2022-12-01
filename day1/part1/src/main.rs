#[derive(Default)]
struct Accumulator {
    max_calories: Option<usize>,
    current_calories: usize,
}

impl Accumulator {
    fn add(mut self, calories: usize) -> Self {
        self.current_calories += calories;
        self
    }

    fn finish(self) -> Self {
        let max_calories = self
            .max_calories
            .map(|max_calories| max_calories.max(self.current_calories))
            .or(Some(self.current_calories));

        Self {
            max_calories,
            current_calories: 0,
        }
    }
}

fn assignment() -> usize {
    let input = include_str!("../../input.txt");
    input
        .lines()
        .fold(Accumulator::default(), |acc, line| {
            if line.is_empty() {
                acc.finish()
            } else {
                acc.add(line.parse().expect("line should contain an integer"))
            }
        })
        .max_calories
        .expect("at least one elf expected")
}

fn main() {
    let max_calories = assignment();
    println!("most calories: {}", max_calories);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let max_calories = assignment();
        assert_eq!(max_calories, 69206);
    }
}
