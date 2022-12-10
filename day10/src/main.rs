mod common;
mod part1;
mod part2;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let answer = part1::run(INPUT);
    println!("part 1: {}", answer);

    let answer = part2::run(INPUT);
    println!("part 2: {}", String::from_utf8_lossy(&answer));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let answer = part1::run(INPUT);
        assert_eq!(answer, 13440);
    }

    #[test]
    fn part2() {
        let answer = part2::run(INPUT);
        assert_eq!(
            answer,
            "
###..###..####..##..###...##..####..##..
#..#.#..#....#.#..#.#..#.#..#....#.#..#.
#..#.###....#..#....#..#.#..#...#..#..#.
###..#..#..#...#.##.###..####..#...####.
#....#..#.#....#..#.#.#..#..#.#....#..#.
#....###..####..###.#..#.#..#.####.#..#."
                .as_bytes()
        );
    }
}
