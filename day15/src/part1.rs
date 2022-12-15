use shared::point::Point;
use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Sensor {
    pos: Point,
    closest_beacon: Point,
}

pub fn parse_sensors(input: &str) -> Vec<Sensor> {
    input.lines().map(parse_sensor).collect()
}

fn parse_sensor(input: &str) -> Sensor {
    let (sensor, baecon) = input.split_once(':').unwrap();

    Sensor {
        pos: parse_coord(sensor),
        closest_beacon: parse_coord(baecon),
    }
}

fn parse_coord(data: &str) -> Point {
    let (x, y) = data.split_once(',').unwrap();
    let x = x.rsplit('=').next().unwrap();
    let y = y.rsplit('=').next().unwrap();

    Point::new(x.parse().unwrap(), y.parse().unwrap())
}

pub fn run(input: &str, row_y: isize) -> usize {
    let sensors = parse_sensors(input);

    let mut ranges = Vec::<RangeInclusive<isize>>::new();
    let mut beacons = HashSet::new();
    for s in &sensors {
        if s.closest_beacon.y == row_y {
            beacons.insert(s.closest_beacon.x);
        }

        let (dx, dy) = (s.closest_beacon - s.pos).into();
        let distance = dx.abs() + dy.abs();

        let dy = distance - (row_y - s.pos.y).abs();
        if dy >= 0 {
            let mut start = s.pos.x - dy;
            let mut end = s.pos.x + dy;

            ranges.retain_mut(|r| {
                if &start <= r.end() && &end >= r.start() {
                    start = start.min(*r.start());
                    end = end.max(*r.end());
                    return false;
                }
                true
            });

            ranges.push(start..=end);
        }
    }

    let mut free = 0;

    for r in ranges {
        free += r.end() - r.start() + 1;
    }

    free as usize - beacons.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        assert_eq!(run(example, 10), 26);
    }
}
