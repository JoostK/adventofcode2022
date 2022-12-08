use crate::common::*;

impl Forest {
    fn is_visible(&self, x: usize, y: usize) -> bool {
        let height = self.tree_height(x, y);
        (0..x).all(|dx| self.tree_height(dx, y) < height)
            || ((x + 1)..self.width).all(|dx| self.tree_height(dx, y) < height)
            || (0..y).all(|dy| self.tree_height(x, dy) < height)
            || ((y + 1)..self.height).all(|dy| self.tree_height(x, dy) < height)
    }
}

pub fn run(input: &str) -> usize {
    let forest = Forest::from(input);

    let mut visible = forest.width * 2 + forest.height * 2 - 4;
    forest.for_each_interior(|(x, y)| {
        if forest.is_visible(x, y) {
            visible += 1;
        }
    });

    visible
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

        assert_eq!(run(example), 21);
    }
}
