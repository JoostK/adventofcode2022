use crate::common::*;

pub fn run(input: &str) -> usize {
    let grid = Grid::from(input);

    let width = grid.width;
    let height = grid.height;
    let (first, grid) = walk(0, width * (height - 1) - 1, grid);
    let (back, grid) = walk(width * height - 1, width, grid);
    let (second, _) = walk(0, width * (height - 1) - 1, grid);

    first + back + second + 1
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

        assert_eq!(run(example), 54);
    }
}
