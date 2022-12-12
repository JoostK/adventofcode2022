use crate::common::*;
use shared::point::Point;
use std::mem::swap;

pub fn search(grid: &Grid, pt: Point, start: Point) -> Option<u16> {
    let mut pending = vec![pt];
    let mut next_pending = Vec::new();
    let mut distance = 0;
    while !pending.is_empty() {
        for &pt in &pending {
            let height = grid.at(pt).height();

            for dir in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                if let Some(neighbour) = grid.neighbour(&pt, dir) {
                    let position = grid.at(neighbour);
                    if height - position.height() > 1 {
                        continue;
                    }
                    if position.mark_visited() {
                        continue;
                    }
                    if neighbour == start {
                        return Some(distance + 1);
                    }
                    next_pending.push(neighbour);
                }
            }
        }

        pending.clear();
        swap(&mut pending, &mut next_pending);
        distance += 1;
    }

    None
}

pub fn run(input: &str) -> usize {
    let (grid, start, end) = parse(input);

    search(&grid, end, start).expect("should find E") as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        assert_eq!(run(example), 31);
    }
}
