use crate::common::*;
use std::collections::HashSet;

pub fn run(input: &str) -> usize {
    let grid = Grid::from(input);

    let (distance, _) = walk(HashSet::from([0]), grid.width * (grid.height - 1) - 1, grid);
    distance + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

        assert_eq!(run(example), 18);
    }
}
