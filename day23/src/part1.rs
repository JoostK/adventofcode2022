use crate::common::Grid;

pub fn run(input: &str) -> usize {
    let mut grid: Grid = input.parse().unwrap();

    for round in 0..10 {
        grid.simulate(round);
    }

    let mut minx = isize::MAX;
    let mut maxx = isize::MIN;
    let mut miny = isize::MAX;
    let mut maxy = isize::MIN;
    for (x, y) in grid.elves() {
        minx = minx.min(x);
        maxx = maxx.max(x);
        miny = miny.min(y);
        maxy = maxy.max(y);
    }

    let width = (maxx - minx + 1) as usize;
    let height = (maxy - miny + 1) as usize;

    width * height - grid.elves().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

        assert_eq!(run(example), 110);
    }
}
