use crate::common::*;

impl Forest {
    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let height = self.tree_height(x, y);

        let mut left = 0;
        for dx in (0..x).rev() {
            left += 1;
            if self.tree_height(dx, y) >= height {
                break;
            }
        }
        let mut right = 0;
        for dx in (x + 1)..self.width {
            right += 1;
            if self.tree_height(dx, y) >= height {
                break;
            }
        }

        let mut up = 0;
        for dy in (0..y).rev() {
            up += 1;
            if self.tree_height(x, dy) >= height {
                break;
            }
        }

        let mut down = 0;
        for dy in (y + 1)..self.height {
            down += 1;
            if self.tree_height(x, dy) >= height {
                break;
            }
        }

        up * down * left * right
    }
}

pub fn run(input: &str) -> usize {
    let forest = Forest::from(input);

    let mut max_score = 0;
    forest.for_each_interior(|(x, y)| {
        let score = forest.scenic_score(x, y);
        max_score = score.max(max_score);
    });

    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "30373
25512
65332
33549
35390";

        assert_eq!(run(example), 8);
    }
}
