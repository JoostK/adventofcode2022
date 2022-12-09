use crate::common::*;

trait CountTrees<T> {
    fn count_trees(&mut self, predicate: impl Fn(T) -> bool) -> usize;
}

impl<T, I> CountTrees<T> for I
where
    I: Iterator<Item = T>,
{
    fn count_trees(&mut self, predicate: impl Fn(T) -> bool) -> usize {
        let mut count = 0;
        for v in self {
            count += 1;
            if !predicate(v) {
                break;
            }
        }
        count
    }
}

impl Forest {
    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let height = self.tree_height(x, y);

        let shorter_x = |dx| self.tree_height(dx, y) < height;
        let shorter_y = |dy| self.tree_height(x, dy) < height;

        self.left_of(x).count_trees(shorter_x)
            * self.right_of(x).count_trees(shorter_x)
            * self.above(y).count_trees(shorter_y)
            * self.below(y).count_trees(shorter_y)
    }
}

pub fn run(input: &str) -> usize {
    let forest = Forest::from(input);

    forest
        .interior()
        .map(|(x, y)| forest.scenic_score(x, y))
        .max()
        .expect("interior should not be empty")
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
