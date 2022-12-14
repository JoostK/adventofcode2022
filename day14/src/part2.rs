use crate::common::*;
use shared::point::Point;

pub fn run(input: &str) -> usize {
    let sand_start = Point::new(500, 0);
    let (mut min, mut max, mut rocks) = parse_rocks(input, sand_start);

    let floor = max.y + 2;
    max.y = floor;

    let spread = floor - sand_start.y;
    let min_x = sand_start.x - spread;
    let max_x = sand_start.x + spread;

    min.x = min.x.min(min_x);
    max.x = max.x.max(max_x);
    rocks.push(Rock {
        lines: vec![(Point::new(min_x, floor), Point::new(max_x, floor))],
    });

    let width = (max.x - min.x + 1) as usize;
    let height = (max.y - min.y + 1) as usize;
    let mut grid = Grid {
        zero: min,
        width,
        height,
        particles: vec![Particle::Air; width * height],
    };

    for rock in rocks {
        for (s, e) in rock.lines {
            let dir = (e - s).map(isize::signum);
            let mut c = s;
            loop {
                *grid.at_mut(c) = Particle::Rock;
                if c == e {
                    break;
                }
                c += dir;
            }
        }
    }

    let mut units = 0;
    while let Some(p) = simulate_sand(&grid, sand_start) {
        *grid.at_mut(p) = Particle::Sand;
        units += 1;

        if p == sand_start {
            break;
        }
    }

    units
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!(run(example), 93);
    }
}
