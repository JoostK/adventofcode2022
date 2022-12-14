use crate::common::*;
use shared::direction::Direction;
use shared::point::Point;
use std::mem::swap;

macro_rules! check_direction {
    ($grid: ident, $start: ident, $pt: ident, $height: ident, $next_pending: ident, $distance: ident, $dir: expr) => {
        if let Some(neighbour) = $grid.neighbour(&$pt, $dir) {
            let position = $grid.at(neighbour);
            let neighbour_height = position.height();
            if $height - neighbour_height <= 1 && !position.mark_visited() {
                if neighbour == $start {
                    return Some($distance + 1);
                }
                $next_pending.push(neighbour);
            }
        }
    };
}

pub fn search(grid: &Grid, pt: Point, start: Point) -> Option<u16> {
    let mut pending = vec![pt];
    let mut next_pending = Vec::new();
    let mut distance = 0;
    while !pending.is_empty() {
        for &pt in &pending {
            let height = grid.at(pt).height();

            check_direction!(
                grid,
                start,
                pt,
                height,
                next_pending,
                distance,
                Direction::Up
            );
            check_direction!(
                grid,
                start,
                pt,
                height,
                next_pending,
                distance,
                Direction::Down
            );
            check_direction!(
                grid,
                start,
                pt,
                height,
                next_pending,
                distance,
                Direction::Left
            );
            check_direction!(
                grid,
                start,
                pt,
                height,
                next_pending,
                distance,
                Direction::Right
            );
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
