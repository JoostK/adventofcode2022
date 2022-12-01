use std::convert::identity;

const MAX_ELVES: usize = 3;

#[derive(Default)]
pub struct Accumulator {
    best_elves: Vec<usize>,
    current_calories: usize,
}

impl super::Assignment for Accumulator {
    fn add(mut self, calories: usize) -> Self {
        self.current_calories += calories;
        self
    }

    fn finish(mut self) -> Self {
        let insertion_index = self
            .best_elves
            .binary_search_by(|elf| self.current_calories.cmp(elf))
            .unwrap_or_else(identity);

        if insertion_index < MAX_ELVES {
            self.best_elves.truncate(MAX_ELVES - 1);
            self.best_elves
                .insert(insertion_index, self.current_calories);
        }

        Self {
            best_elves: self.best_elves,
            current_calories: 0,
        }
    }

    fn total_calories(&self) -> usize {
        self.best_elves.iter().sum()
    }
}
