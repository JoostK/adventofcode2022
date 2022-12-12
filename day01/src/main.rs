use day01::*;

fn main() {
    let max_calories = run::<part1::Accumulator>();
    println!("part 1: {}", max_calories);

    let max_calories = run::<part2::Accumulator>();
    println!("part 2: {}", max_calories);
}
