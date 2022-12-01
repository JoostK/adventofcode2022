#[derive(Default)]
pub struct Accumulator {
    max_calories: Option<usize>,
    current_calories: usize,
}

impl super::Assignment for Accumulator {
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

    fn total_calories(&self) -> usize {
        self.max_calories.expect("at least one elf expected")
    }
}
